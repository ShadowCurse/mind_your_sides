use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{utils::remove_all_with, GlobalState};

use self::spawn::EnemySpawnBuffs;

use super::{
    castle::{Castle, CastleWall},
    East, North, Side, South, West,
};

mod spawn;

/// Needed to make enemies move.
/// Otherwise we would need set enormous speeds.
const ENEMY_FORCE_MULTIPLIER: f32 = 1000.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, EnemySprites>(GlobalState::AssetLoading)
            .add_plugin(spawn::SpawnPlugin)
            .add_systems(
                (
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

#[derive(AssetCollection, Resource)]
struct EnemySprites {
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 4, rows = 1,))]
    #[asset(path = "images/goblin.png")]
    pub goblin: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 4, rows = 1,))]
    #[asset(path = "images/spear_goblin.png")]
    pub spear_goblin: Handle<TextureAtlas>,
}

#[derive(Debug, Default, Component)]
pub struct Enemy {
    pub health: i32,
    pub speed: f32,
    pub exp: u32,
}

#[derive(Debug, Default, Component)]
pub struct EnemyMarker;

#[derive(Default, Bundle)]
pub struct EnemyBundle<S: Side, E: EnemyType> {
    rigid_body: RigidBody,
    collider: Collider,
    velocity: Velocity,
    damping: Damping,
    #[bundle]
    sprite: SpriteSheetBundle,
    enemy: Enemy,
    side: S,
    enemy_type: E,
    marker: EnemyMarker,
}

impl<S: Side, E: EnemyType> EnemyBundle<S, E> {
    fn new(size: f32, sprite: Handle<TextureAtlas>, position: Vec3, buffs: &EnemySpawnBuffs) -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(size),
            velocity: Velocity::default(),
            damping: Damping {
                linear_damping: 5.0,
                angular_damping: 10.0,
            },

            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..default()
                },
                texture_atlas: sprite,
                transform: Transform::from_translation(position),
                ..default()
            },
            enemy: E::enemy(buffs),
            side: S::default(),
            enemy_type: E::default(),
            marker: EnemyMarker,
        }
    }
}

pub trait EnemyType: Component + Default {
    const HEALTH: i32;
    const SPEED: f32;
    const EXP: u32;

    fn enemy(buffs: &EnemySpawnBuffs) -> Enemy {
        Enemy {
            health: (Self::HEALTH as f32 * buffs.health) as i32,
            speed: Self::SPEED * buffs.speed,
            exp: (Self::EXP as f32 * buffs.exp) as u32,
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct Goblin;

impl EnemyType for Goblin {
    const HEALTH: i32 = 80;
    const SPEED: f32 = 20.0;
    const EXP: u32 = 50;
}

#[derive(Debug, Default, Component)]
pub struct SpearGoblin;

impl EnemyType for SpearGoblin {
    const HEALTH: i32 = 100;
    const SPEED: f32 = 15.0;
    const EXP: u32 = 80;
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

        let movement = direction * time.delta().as_secs_f32();
        enemy_velocity.linvel = movement * enemy.speed * ENEMY_FORCE_MULTIPLIER;
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
