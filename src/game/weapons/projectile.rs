use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    game::{damage::EnemyDamageEvent, enemies::Enemy, East, North, Side, South, West},
    GlobalState,
};

use super::DamageMarker;

const DEFAULT_ARROW_SIZE: f32 = 3.0;
const DEFAULT_ARROW_DAMAGE: i32 = 20;
const DEFAULT_ARROW_SPEED: f32 = 200.0;
const DEFAULT_ARROW_LIFESPAN: f32 = 10.0;

const DEFAULT_ARCHER_RANGE: f32 = 200.0;
const DEFAULT_ARCHER_ATTACK_SPEED: f32 = 1.0;
/// Offsets arrow spawn point in the enemy direction
const DEFAULT_ARROW_SPAWN_OFFSET: f32 = 30.0;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                archer_attack::<North>,
                archer_attack::<South>,
                archer_attack::<West>,
                archer_attack::<East>,
                projectile_update::<North>,
                projectile_update::<South>,
                projectile_update::<West>,
                projectile_update::<East>,
            )
                .in_set(OnUpdate(GlobalState::InGame)),
        );
    }
}

#[derive(Component)]
pub struct Archer {
    damage: i32,
    range: f32,
    arrow_speed: f32,
    attack_timer: Timer,
}

impl Default for Archer {
    fn default() -> Self {
        Self {
            damage: DEFAULT_ARROW_DAMAGE,
            range: DEFAULT_ARCHER_RANGE,
            arrow_speed: DEFAULT_ARROW_SPEED,
            attack_timer: Timer::from_seconds(DEFAULT_ARCHER_ATTACK_SPEED, TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct Projectile {
    damage: i32,
    lifespan: Timer,
}

#[derive(Bundle)]
pub struct ProjectileBundle<S: Side> {
    rigid_body: RigidBody,
    collider: Collider,
    velocity: Velocity,
    projectile: Projectile,
    side: S,
    marker: DamageMarker,
}

impl<S: Side> ProjectileBundle<S> {
    fn new(size: f32, damage: i32, speed: f32, direction: Vec2) -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(size),
            velocity: Velocity {
                linvel: speed * direction,
                ..default()
            },
            projectile: Projectile {
                damage,
                lifespan: Timer::from_seconds(DEFAULT_ARROW_LIFESPAN, TimerMode::Once),
            },
            side: S::default(),
            marker: DamageMarker,
        }
    }
}

fn archer_attack<S: Side>(
    time: Res<Time>,
    enemies: Query<&Transform, (With<Enemy>, With<S>)>,
    mut commands: Commands,
    mut archers: Query<(&Transform, &mut Archer), With<S>>,
) {
    for (transform, mut archer) in archers.iter_mut() {
        if !archer.attack_timer.tick(time.delta()).finished() {
            continue;
        }

        let mut enemy_vec = Vec2::default();
        let mut min_range = archer.range;
        for enemy_transform in enemies.iter() {
            let vec = (enemy_transform.translation - transform.translation).truncate();
            let distance = vec.length();
            if distance < min_range {
                min_range = distance;
                enemy_vec = vec;
            }
        }

        // no enemies in range
        if archer.range <= min_range {
            continue;
        }

        let direction = enemy_vec.normalize();
        let mut spawn_point = *transform;
        spawn_point.translation += (direction * DEFAULT_ARROW_SPAWN_OFFSET).extend(0.0);
        commands
            .spawn(TransformBundle::from_transform(spawn_point))
            .insert(ProjectileBundle::<S>::new(
                DEFAULT_ARROW_SIZE,
                archer.damage,
                archer.arrow_speed,
                direction,
            ));
    }
}

fn projectile_update<S: Side>(
    time: Res<Time>,
    enemies: Query<Entity, (With<Enemy>, With<S>)>,
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
    mut projectiles: Query<(Entity, &mut Projectile), With<S>>,
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
