use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::{utils::remove_all_with, GlobalState};

use super::{
    castle::{Castle, CastleWall},
    East, North, Side, South, West,
};

/// Needed to make enemies move.
/// Otherwise we would need set enormous speeds.
const ENEMY_FORCE_MULTIPLIER: f32 = 1000.0;

const DEFAULT_ENEMY_SPAWN_NUMBER: u32 = 1;
const DEFAULT_ENEMY_SPAWN_RADIUS: f32 = 200.0;
const DEFAULT_ENEMY_SPAWN_RATE: f32 = 5.0;

const DEFAULT_ENEMY_SIZE: f32 = 10.0;
const DEFAULT_ENEMY_HEALTH: i32 = 100;
const DEFAULT_ENEMY_SPEED: f32 = 10.0;
const DEFAULT_ENEMY_EXP: u32 = 10;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GlobalState::InGame)))
            .add_systems(
                (
                    enemy_spawn::<North>,
                    enemy_spawn::<South>,
                    enemy_spawn::<West>,
                    enemy_spawn::<East>,
                    enemy_movement::<North>,
                    enemy_movement::<South>,
                    enemy_movement::<West>,
                    enemy_movement::<East>,
                    enemy_death::<North>,
                    enemy_death::<South>,
                    enemy_death::<West>,
                    enemy_death::<East>,
                )
                    .in_set(OnUpdate(GlobalState::InGame)),
            )
            .add_system(remove_all_with::<EnemyMarker>.in_schedule(OnEnter(GlobalState::MainMenu)));
    }
}

#[derive(Component)]
pub struct Enemy {
    pub health: i32,
    pub speed: f32,
    pub exp: u32,
}

#[derive(Component)]
pub struct EnemyMarker;

#[derive(Component)]
pub struct EnemySpawn<S: Side> {
    pub number: u32,
    pub radius: f32,
    pub timer: Timer,
    pub side: S,
}

impl<S: Side> Default for EnemySpawn<S> {
    fn default() -> Self {
        Self {
            number: DEFAULT_ENEMY_SPAWN_NUMBER,
            radius: DEFAULT_ENEMY_SPAWN_RADIUS,
            timer: Timer::from_seconds(DEFAULT_ENEMY_SPAWN_RATE, TimerMode::Repeating),
            side: S::default(),
        }
    }
}

#[derive(Component)]
pub struct Experience {
    pub exp: u32,
}

#[derive(Bundle)]
pub struct EnemyBundle<S: Side> {
    rigid_body: RigidBody,
    collider: Collider,
    velocity: Velocity,
    damping: Damping,
    enemy: Enemy,
    side: S,
    marker: EnemyMarker,
}

impl<S: Side> Default for EnemyBundle<S> {
    fn default() -> Self {
        Self::new(
            DEFAULT_ENEMY_SIZE,
            DEFAULT_ENEMY_HEALTH,
            DEFAULT_ENEMY_SPEED,
            DEFAULT_ENEMY_EXP,
        )
    }
}

impl<S: Side> EnemyBundle<S> {
    fn new(size: f32, health: i32, speed: f32, exp: u32) -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(size),
            velocity: Velocity::default(),
            damping: Damping {
                linear_damping: 5.0,
                angular_damping: 10.0,
            },
            enemy: Enemy { health, speed, exp },
            side: S::default(),
            marker: EnemyMarker,
        }
    }
}

// TODO replace with sprites
#[derive(Resource)]
struct EnemyMeshMaterial {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

/// Sets up 4 spawns at each side of the screen
fn setup(
    mut commands: Commands,
    // TODO replace with sprites
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let spawn_mesh = meshes.add(shape::Circle::new(15.0).into());
    let spawn_material = materials.add(ColorMaterial::from(Color::ORANGE));

    // North
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: spawn_mesh.clone().into(),
            material: spawn_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 500.0, 0.0)),
            ..default()
        })
        .insert(EnemySpawn::<North>::default());
    // South
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: spawn_mesh.clone().into(),
            material: spawn_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -500.0, 0.0)),
            ..default()
        })
        .insert(EnemySpawn::<South>::default());
    // West
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: spawn_mesh.clone().into(),
            material: spawn_material.clone(),
            transform: Transform::from_translation(Vec3::new(-500.0, 0.0, 0.0)),
            ..default()
        })
        .insert(EnemySpawn::<West>::default());
    // East
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: spawn_mesh.into(),
            material: spawn_material,
            transform: Transform::from_translation(Vec3::new(500.0, 0.0, 0.0)),
            ..default()
        })
        .insert(EnemySpawn::<East>::default());

    let enemy_mesh = meshes.add(shape::Circle::new(10.0).into());
    let enemy_material = materials.add(ColorMaterial::from(Color::RED));

    commands.insert_resource(EnemyMeshMaterial {
        mesh: enemy_mesh,
        material: enemy_material,
    });
}

/// Spawns enemies in a circle arond the spawn point equally spread
/// on a circle
fn enemy_spawn<S: Side>(
    time: Res<Time>,
    mesh_material: Res<EnemyMeshMaterial>,
    mut commands: Commands,
    mut spawns: Query<(&Transform, &mut EnemySpawn<S>)>,
) {
    for (transform, mut spawn) in spawns.iter_mut() {
        if !spawn.timer.tick(time.delta()).finished() {
            continue;
        }

        for n in 0..spawn.number {
            let position = transform.translation
                + Quat::from_rotation_z(
                    (2.0 * std::f32::consts::PI / spawn.number as f32) * n as f32,
                )
                .mul_vec3(Vec3::Y * spawn.radius);

            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: mesh_material.mesh.clone().into(),
                    material: mesh_material.material.clone(),
                    transform: Transform::from_translation(position),
                    ..default()
                })
                .insert(EnemyBundle::<S>::default());
        }
    }
}

/// Moved enemies in direction of the wall
/// Keeps them pointed at the wall
fn enemy_movement<S: Side>(
    time: Res<Time>,
    wall: Query<&Transform, (With<CastleWall>, With<S>)>,
    mut enemies: Query<(&Transform, &Enemy, &mut Velocity), With<S>>,
) {
    let wall_transform = wall.single();

    for (enemy_transform, enemy, mut enemy_velocity) in enemies.iter_mut() {
        let vector = (wall_transform.translation - enemy_transform.translation).truncate();
        let direction = vector.normalize();

        // calculate cos between movement direction and direction enemy is looking at
        // we set the angvel to -cos to ratote enemies X axis in movement direction
        let enemy_direction = enemy_transform
            .rotation
            .mul_vec3(Vec3::X)
            .truncate()
            .normalize();
        let cos = direction.dot(enemy_direction);

        let movement = direction * time.delta().as_secs_f32();
        enemy_velocity.linvel = movement * enemy.speed * ENEMY_FORCE_MULTIPLIER;
        enemy_velocity.angvel = -cos;
    }
}

fn enemy_death<S: Side>(
    enemies: Query<(Entity, &Enemy), With<S>>,
    mut commands: Commands,
    mut castle: Query<&mut Castle>,
) {
    let mut castle = castle.single_mut();
    for (enemy_entity, enemy) in enemies.iter() {
        if enemy.health <= 0 {
            castle.exp += enemy.exp;
            commands.entity(enemy_entity).despawn();
        }
    }
}
