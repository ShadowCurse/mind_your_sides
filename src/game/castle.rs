use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::{utils::remove_all_with, GlobalState};

use super::{
    weapons::{area::Catapulte, projectile::Archer},
    East, GameState, North, Side, South, West,
};

const WALL_LENGTH: f32 = 100.0;
const WALL_THICKNESS: f32 = 10.0;
const WALL_HEALTH: i32 = 100;

const CASTLE_FIRST_LEVEL_EXP: u32 = 100;
const CASTLE_NEXT_LEVEL_EXP_GROWTH: f32 = 1.1;

pub struct CastlePlugin;

impl Plugin for CastlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GlobalState::InGame)))
            .add_system(castle_level_up.in_set(OnUpdate(GlobalState::InGame)))
            .add_system(remove_all_with::<CastleMarker>.in_schedule(OnExit(GlobalState::InGame)))
            .add_system(
                remove_all_with::<CastleWallMarker>.in_schedule(OnExit(GlobalState::InGame)),
            );
    }
}

#[derive(Component)]
pub struct Castle {
    pub level: u32,
    pub exp: u32,
    pub next_level_exp: u32,
    pub next_level_exp_growth: f32,
}

#[derive(Component)]
pub struct CastleMarker;

#[derive(Bundle)]
pub struct CastleBundle {
    castle: Castle,
    marker: CastleMarker,
}

impl Default for CastleBundle {
    fn default() -> Self {
        Self {
            castle: Castle {
                level: 0,
                exp: 0,
                next_level_exp: CASTLE_FIRST_LEVEL_EXP,
                next_level_exp_growth: CASTLE_NEXT_LEVEL_EXP_GROWTH,
            },
            marker: CastleMarker,
        }
    }
}

#[derive(Component)]
pub struct CastleWall {
    pub health: i32,
}

#[derive(Component)]
pub struct CastleWallMarker;

#[derive(Bundle)]
pub struct CastleWallBundle<S: Side> {
    rigid_body: RigidBody,
    collider: Collider,
    wall: CastleWall,
    side: S,
    marker: CastleWallMarker,
}

impl<S: Side> CastleWallBundle<S> {
    fn new(health: i32, length: f32, thickness: f32) -> Self {
        Self {
            rigid_body: RigidBody::Fixed,
            collider: Collider::cuboid(length, thickness),
            wall: CastleWall { health },
            side: S::default(),
            marker: CastleWallMarker,
        }
    }
}

/// Sets up castle in the center of the map
/// with 4 walls
fn setup(
    mut commands: Commands,
    // TODO replace with sprites
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let castle_mesh = meshes.add(shape::Box::new(WALL_LENGTH, WALL_LENGTH, 0.0).into());
    let castle_material = materials.add(ColorMaterial::from(Color::BLUE));
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: castle_mesh.into(),
            material: castle_material,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        })
        .insert(CastleBundle::default());

    let horizontal_wall_mesh = meshes.add(shape::Box::new(WALL_LENGTH, WALL_THICKNESS, 0.0).into());
    let vertical_wall_mesh = meshes.add(shape::Box::new(WALL_THICKNESS, WALL_LENGTH, 0.0).into());
    let wall_material = materials.add(ColorMaterial::from(Color::DARK_GRAY));

    // Walls have z of 1.0 to fix z fighting
    // North
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: horizontal_wall_mesh.clone().into(),
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 50.0, 1.0)),
            ..default()
        })
        .insert(CastleWallBundle::<North>::new(
            WALL_HEALTH,
            WALL_LENGTH / 2.0,
            WALL_THICKNESS / 2.0,
        ))
        .insert(Archer::default())
        .insert(Catapulte::default());
    // South
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: horizontal_wall_mesh.into(),
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -50.0, 1.0)),
            ..default()
        })
        .insert(CastleWallBundle::<South>::new(
            WALL_HEALTH,
            WALL_LENGTH / 2.0,
            WALL_THICKNESS / 2.0,
        ))
        .insert(Archer::default())
        .insert(Catapulte::default());
    // West
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: vertical_wall_mesh.clone().into(),
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(-50.0, 0.0, 1.0)),
            ..default()
        })
        .insert(CastleWallBundle::<West>::new(
            WALL_HEALTH,
            WALL_THICKNESS / 2.0,
            WALL_LENGTH / 2.0,
        ))
        .insert(Archer::default())
        .insert(Catapulte::default());
    // East
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: vertical_wall_mesh.into(),
            material: wall_material,
            transform: Transform::from_translation(Vec3::new(50.0, 0.0, 1.0)),
            ..default()
        })
        .insert(CastleWallBundle::<East>::new(
            WALL_HEALTH,
            WALL_THICKNESS / 2.0,
            WALL_LENGTH / 2.0,
        ))
        .insert(Archer::default())
        .insert(Catapulte::default());
}

fn castle_level_up(mut castle: Query<&mut Castle>, mut game_state: ResMut<NextState<GameState>>) {
    let mut castle = castle.single_mut();

    if castle.exp >= castle.next_level_exp {
        castle.level += 1;
        castle.exp -= castle.next_level_exp;
        castle.next_level_exp =
            (castle.next_level_exp as f32 * castle.next_level_exp_growth) as u32;

        game_state.set(GameState::LevelUp);
    }
}
