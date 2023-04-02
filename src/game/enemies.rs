use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::{utils::remove_all_with, GlobalState};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GlobalState::InGame)))
            .add_system(enemy_spawn.in_set(OnUpdate(GlobalState::InGame)))
            .add_system(remove_all_with::<EnemyMarker>.in_schedule(OnEnter(GlobalState::MainMenu)));
    }
}

#[derive(Component)]
pub struct Enemy {
    pub health: i32,
    pub speed: f32,
}

#[derive(Component)]
pub struct EnemyMarker;

#[derive(Component)]
pub struct EnemySpawn {
    pub number: u32,
    pub radius: f32,
    pub timer: Timer,
}

#[derive(Component)]
pub struct Experience {
    pub exp: u32,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    rigid_body: RigidBody,
    collider: Collider,
    velocity: Velocity,
    damping: Damping,
    enemy: Enemy,
    marker: EnemyMarker,
}

impl EnemyBundle {
    fn new(size: f32, health: i32, speed: f32) -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(size),
            velocity: Velocity::default(),
            damping: Damping {
                linear_damping: 10.0,
                angular_damping: 1.0,
            },
            enemy: Enemy { health, speed },
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
        .insert(EnemySpawn {
            number: 10,
            radius: 100.0,
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        });
    // South
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: spawn_mesh.clone().into(),
            material: spawn_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -500.0, 0.0)),
            ..default()
        })
        .insert(EnemySpawn {
            number: 10,
            radius: 100.0,
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        });
    // West
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: spawn_mesh.clone().into(),
            material: spawn_material.clone(),
            transform: Transform::from_translation(Vec3::new(-500.0, 0.0, 0.0)),
            ..default()
        })
        .insert(EnemySpawn {
            number: 10,
            radius: 100.0,
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        });
    // East
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: spawn_mesh.into(),
            material: spawn_material,
            transform: Transform::from_translation(Vec3::new(500.0, 0.0, 0.0)),
            ..default()
        })
        .insert(EnemySpawn {
            number: 10,
            radius: 100.0,
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        });

    let enemy_mesh = meshes.add(shape::Circle::new(10.0).into());
    let enemy_material = materials.add(ColorMaterial::from(Color::RED));

    commands.insert_resource(EnemyMeshMaterial {
        mesh: enemy_mesh,
        material: enemy_material,
    });
}

fn enemy_spawn(
    time: Res<Time>,
    mesh_material: Res<EnemyMeshMaterial>,
    mut commands: Commands,
    mut spawns: Query<(&Transform, &mut EnemySpawn)>,
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
                .insert(EnemyBundle::new(10.0, 10, 10.0));
        }
    }
}
