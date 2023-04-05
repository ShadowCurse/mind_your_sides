use std::marker::PhantomData;

use bevy::{prelude::*, sprite::Anchor};
use bevy_asset_loader::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{utils::remove_all_with, GlobalState};

use super::{
    weapons::{crossbow::CrossbowBundle, molotov::MolotovBundle},
    East, GameState, North, Side, South, West,
};

const WALL_HEALTH: i32 = 100;

// TODO: Use in the refactor
// const WALL_LENGTH: f32 = 256.0;
// const WALL_THICKNESS: f32 = 16.0;

const CASTLE_FIRST_LEVEL_EXP: u32 = 100;
const CASTLE_NEXT_LEVEL_EXP_GROWTH: f32 = 1.1;

pub struct CastlePlugin;

impl Plugin for CastlePlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, CastleSprites>(GlobalState::AssetLoading)
            .add_system(setup.in_schedule(OnEnter(GlobalState::InGame)))
            .add_systems(
                (
                    castle_level_up,
                    check_wall_destroyed::<North>,
                    check_wall_destroyed::<South>,
                    check_wall_destroyed::<West>,
                    check_wall_destroyed::<East>,
                )
                    .in_set(OnUpdate(GameState::InGame)),
            )
            .add_system(remove_all_with::<CastleMarker>.in_schedule(OnExit(GlobalState::InGame)))
            .add_system(
                remove_all_with::<CastleWallMarker>.in_schedule(OnExit(GlobalState::InGame)),
            );
    }
}

#[derive(AssetCollection, Resource)]
pub struct CastleSprites {
    #[asset(path = "sprites/castle.png")]
    pub castle: Handle<Image>,
    #[asset(path = "sprites/wall_north.png")]
    pub wall_north: Handle<Image>,
    #[asset(path = "sprites/wall_south.png")]
    pub wall_south: Handle<Image>,
    #[asset(path = "sprites/wall_west.png")]
    pub wall_west: Handle<Image>,
    #[asset(path = "sprites/wall_east.png")]
    pub wall_east: Handle<Image>,
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
pub struct CastleWall<S: Side> {
    pub health: i32,
    _phantom: PhantomData<S>,
}

impl<S: Side> CastleWall<S> {
    pub fn new(health: i32) -> Self {
        Self {
            health,
            _phantom: PhantomData,
        }
    }
}

#[derive(Component)]
pub struct CastleWallMarker;

#[derive(Bundle)]
pub struct CastleWallBundle<S: Side> {
    rigid_body: RigidBody,
    collider: Collider,
    wall: CastleWall<S>,
    #[bundle]
    crossbow: CrossbowBundle<S>,
    #[bundle]
    molotov: MolotovBundle<S>,
    marker: CastleWallMarker,
}

impl<S: Side> CastleWallBundle<S> {
    fn new(health: i32, length: f32, thickness: f32) -> Self {
        Self {
            rigid_body: RigidBody::Fixed,
            collider: Collider::cuboid(length, thickness),
            wall: CastleWall::new(health),
            crossbow: Default::default(),
            molotov: Default::default(),
            marker: CastleWallMarker,
        }
    }
}

/// Sets up castle in the center of the map
/// with 4 walls
fn setup(castle_sprites: Res<CastleSprites>, mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 3.0)),
            texture: castle_sprites.castle.clone(),
            ..default()
        })
        .insert(CastleBundle::default());

    // TODO: Refactor hardcoded values
    // North
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 386.0, y: 92.0 }),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 100.0, 2.0)),
            texture: castle_sprites.wall_north.clone(),
            ..default()
        })
        .insert(CastleWallBundle::<North>::new(
            WALL_HEALTH,
            386.0 / 2.0,
            24.0 / 2.0,
        ));
    // South
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 386.0, y: 92.0 }),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -193.0, 4.0)),
            texture: castle_sprites.wall_south.clone(),
            ..default()
        })
        .insert(CastleWallBundle::<South>::new(
            WALL_HEALTH,
            386.0 / 2.0,
            24.0 / 2.0,
        ));
    // West
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 24.0, y: 386.0 }),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-204.0, 0.0, 3.0)),
            texture: castle_sprites.wall_west.clone(),
            ..default()
        })
        .insert(CastleWallBundle::<West>::new(
            WALL_HEALTH,
            24.0 / 2.0,
            386.0 / 2.0,
        ));
    // East
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 24.0, y: 386.0 }),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(204.0, 0.0, 3.0)),
            texture: castle_sprites.wall_east.clone(),
            ..default()
        })
        .insert(CastleWallBundle::<East>::new(
            WALL_HEALTH,
            24.0 / 2.0,
            386.0 / 2.0,
        ));
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

fn check_wall_destroyed<S: Side>(
    wall: Query<&CastleWall<S>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let wall = wall.single();
    if wall.health <= 0 {
        game_state.set(GameState::GameOver);
    }
}
