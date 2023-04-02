use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{utils::remove_all_with, GlobalState};

use super::{enemies::Enemy, East, North, Side, South, West};

const DEFAULT_ARROW_SIZE: f32 = 3.0;
const DEFAULT_ARROW_DAMAGE: i32 = 10;
const DEFAULT_ARROW_SPEED: f32 = 200.0;
const DEFAULT_ARROW_LIFESPAN: f32 = 10.0;

const DEFAULT_ARCHER_RANGE: f32 = 400.0;
const DEFAULT_ARCHER_ATTACK_SPEED: f32 = 1.0;
/// Offsets arrow spawn point in the enemy direction
const DEFAULT_ARROW_SPAWN_OFFSET: f32 = 30.0;

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                archer_attack::<North>,
                archer_attack::<South>,
                archer_attack::<West>,
                archer_attack::<East>,
                arrow_update::<North>,
                arrow_update::<South>,
                arrow_update::<West>,
                arrow_update::<East>,
            )
                .in_set(OnUpdate(GlobalState::InGame)),
        )
        .add_system(remove_all_with::<ProjectileMarker>.in_schedule(OnExit(GlobalState::InGame)));
    }
}

#[derive(Component)]
pub struct ProjectileMarker;

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
pub struct Arrow {
    damage: i32,
    lifespan: Timer,
}

#[derive(Bundle)]
pub struct ArrowBundle<S: Side> {
    rigid_body: RigidBody,
    collider: Collider,
    velocity: Velocity,
    arrow: Arrow,
    side: S,
    marker: ProjectileMarker,
}

impl<S: Side> ArrowBundle<S> {
    fn new(size: f32, damage: i32, speed: f32, direction: Vec2) -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(size),
            velocity: Velocity {
                linvel: speed * direction,
                ..default()
            },
            arrow: Arrow {
                damage,
                lifespan: Timer::from_seconds(DEFAULT_ARROW_LIFESPAN, TimerMode::Once),
            },
            side: S::default(),
            marker: ProjectileMarker,
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

        info!("Spawning arrow");
        let direction = enemy_vec.normalize();
        let mut spawn_point = *transform;
        spawn_point.translation += (direction * DEFAULT_ARROW_SPAWN_OFFSET).extend(0.0);
        commands
            .spawn(TransformBundle::from_transform(spawn_point))
            .insert(ArrowBundle::<S>::new(
                DEFAULT_ARROW_SIZE,
                archer.damage,
                archer.arrow_speed,
                direction,
            ));
    }
}

fn arrow_update<S: Side>(
    time: Res<Time>,
    enemies: Query<Entity, (With<Enemy>, With<S>)>,
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
    mut arrows: Query<(Entity, &mut Arrow), With<S>>,
) {
    for (arrow_entity, mut arrow) in arrows.iter_mut() {
        if arrow.lifespan.tick(time.delta()).finished() {
            commands.entity(arrow_entity).despawn();
        } else {
            let mut hit = false;
            for contact_pair in rapier_context.contacts_with(arrow_entity) {
                if let Ok(_enemy) = enemies
                    .get(contact_pair.collider1())
                    .or(enemies.get(contact_pair.collider2()))
                {
                    hit = true;
                }
            }
            if hit {
                commands.entity(arrow_entity).despawn();
            }
        }
    }
}
