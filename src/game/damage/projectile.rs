use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    game::{damage::EnemyDamageEvent, enemies::Enemy, East, GameState, North, Side, South, West},
    utils::remove_all_with,
    GlobalState,
};

const DEFAULT_ARROW_LIFESPAN: f32 = 10.0;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                projectile_update::<North>,
                projectile_update::<South>,
                projectile_update::<West>,
                projectile_update::<East>,
            )
                .in_set(OnUpdate(GameState::InGame)),
        )
        .add_system(remove_all_with::<ProjectileMarker>.in_schedule(OnExit(GlobalState::InGame)));
    }
}

#[derive(Component)]
pub struct ProjectileMarker;

#[derive(Component)]
pub struct Projectile<S: Side> {
    damage: i32,
    lifespan: Timer,
    _phantom: PhantomData<S>,
}

impl<S: Side> Projectile<S> {
    pub fn new(damage: i32, lifespan: f32) -> Self {
        Self {
            damage,
            lifespan: Timer::from_seconds(lifespan, TimerMode::Once),
            _phantom: PhantomData,
        }
    }
}

#[derive(Bundle)]
pub struct ProjectileBundle<S: Side> {
    #[bundle]
    sprite: SpriteBundle,
    rigid_body: RigidBody,
    collider: Collider,
    velocity: Velocity,
    projectile: Projectile<S>,
    marker: ProjectileMarker,
}

impl<S: Side> ProjectileBundle<S> {
    pub fn new(
        texture: Handle<Image>,
        size: f32,
        damage: i32,
        speed: f32,
        direction: Vec2,
        transform: Transform,
    ) -> Self {
        Self {
            sprite: SpriteBundle {
                texture,
                transform,
                ..default()
            },
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(size),
            velocity: Velocity {
                linvel: speed * direction,
                ..default()
            },
            projectile: Projectile::new(damage, DEFAULT_ARROW_LIFESPAN),
            marker: ProjectileMarker,
        }
    }
}

fn projectile_update<S: Side>(
    time: Res<Time>,
    enemies: Query<Entity, With<Enemy<S>>>,
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
    mut projectiles: Query<(Entity, &mut Projectile<S>)>,
    mut damage_event: EventWriter<EnemyDamageEvent<S>>,
) {
    for (projectile_entity, mut projectile) in projectiles.iter_mut() {
        if projectile.lifespan.tick(time.delta()).finished() {
            commands.entity(projectile_entity).despawn();
        } else {
            let mut hit = false;
            for contact_pair in rapier_context.contacts_with(projectile_entity) {
                if let Ok(enemy) = enemies
                    .get(contact_pair.collider1())
                    .or(enemies.get(contact_pair.collider2()))
                {
                    hit = true;
                    damage_event.send(EnemyDamageEvent::new(enemy, projectile.damage));
                }
            }
            if hit {
                commands.entity(projectile_entity).despawn();
            }
        }
    }
}
