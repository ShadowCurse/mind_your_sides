use std::{fmt::Display, marker::PhantomData};

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{utils::remove_all_with, GlobalState};

use self::spawn::EnemyBuffs;

use super::{
    animation::AnimationBundle,
    castle::{Castle, CastleWall},
    damage::WallDamageEvent,
    East, GameState, North, Side, South, West,
};

pub mod spawn;

/// Needed to make enemies move.
/// Otherwise we would need set enormous speeds.
#[cfg(target_family = "wasm")]
const ENEMY_FORCE_MULTIPLIER: f32 = 300.0;
#[cfg(not(target_family = "wasm"))]
const ENEMY_FORCE_MULTIPLIER: f32 = 1000.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SpawnState>()
            .add_collection_to_loading_state::<_, EnemySprites>(GlobalState::AssetLoading)
            .add_system(setup.in_schedule(OnEnter(GlobalState::InGame)))
            .add_systems(
                (
                    enemy_spawn_state_progress,
                    enemy_movement::<North>,
                    enemy_movement::<South>,
                    enemy_movement::<West>,
                    enemy_movement::<East>,
                    enemy_attack::<North>,
                    enemy_attack::<South>,
                    enemy_attack::<West>,
                    enemy_attack::<East>,
                    enemy_death::<North>,
                    enemy_death::<South>,
                    enemy_death::<West>,
                    enemy_death::<East>,
                )
                    .in_set(OnUpdate(GameState::InGame)),
            )
            .add_system(remove_all_with::<EnemyMarker>.in_schedule(OnExit(GlobalState::InGame)))
            .add_plugin(spawn::SpawnPlugin::<North>::default())
            .add_plugin(spawn::SpawnPlugin::<South>::default())
            .add_plugin(spawn::SpawnPlugin::<West>::default())
            .add_plugin(spawn::SpawnPlugin::<East>::default());
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum SpawnState {
    #[default]
    // Bats and Gablin
    // 1 min
    Stage1,
    // + Skull and Spear goblin
    // 2 min
    Stage2,
    // + Huge ivy
    // 3 min
    Stage3,
    // + Boss crab
    Stage4,
}

impl Display for SpawnState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpawnState::Stage1 => f.write_str("1"),
            SpawnState::Stage2 => f.write_str("2"),
            SpawnState::Stage3 => f.write_str("3"),
            SpawnState::Stage4 => f.write_str("Final"),
        }
    }
}

#[derive(Debug, Resource)]
pub struct SpawnStateTimer {
    pub current_state: SpawnState,
    pub timer: Option<Timer>,
}

impl Default for SpawnStateTimer {
    fn default() -> Self {
        Self {
            current_state: Default::default(),
            timer: Some(Timer::from_seconds(Self::FIST_STAGE, TimerMode::Once)),
        }
    }
}

impl SpawnStateTimer {
    const FIST_STAGE: f32 = 60.0;
    const SECOND_STAGE: f32 = 60.0 * 2.0;
    const THIRD_STAGE: f32 = 60.0 * 2.0;

    fn next_state(&mut self) {
        match self.current_state {
            SpawnState::Stage1 => {
                self.current_state = SpawnState::Stage2;
                self.timer = Some(Timer::from_seconds(Self::SECOND_STAGE, TimerMode::Once));
            }
            SpawnState::Stage2 => {
                self.current_state = SpawnState::Stage3;
                self.timer = Some(Timer::from_seconds(Self::THIRD_STAGE, TimerMode::Once));
            }
            SpawnState::Stage3 => {
                self.current_state = SpawnState::Stage4;
                self.timer = None;
            }
            _ => {}
        }
    }
}

#[derive(AssetCollection, Resource)]
pub struct EnemySprites {
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 4, rows = 1,))]
    #[asset(path = "sprites/mad_crab.png")]
    pub mad_crab: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 4, rows = 1,))]
    #[asset(path = "sprites/goblin.png")]
    pub goblin: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 4, rows = 1,))]
    #[asset(path = "sprites/spear_goblin.png")]
    pub spear_goblin: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 4, rows = 1,))]
    #[asset(path = "sprites/bat.png")]
    pub bat: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 4, rows = 1,))]
    #[asset(path = "sprites/skull.png")]
    pub skull: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 4, rows = 1,))]
    #[asset(path = "sprites/poison_ivy.png")]
    pub poison_ivy: Handle<TextureAtlas>,
}

#[derive(Debug, Default, Resource)]
pub struct GlobalEnemyBuffs {
    pub health: f32,
    pub speed: f32,
    pub exp: f32,
    pub damage: f32,
    pub attack_speed: f32,
}

#[derive(Debug, Default, Component)]
pub struct Enemy<S: Side> {
    pub health: i32,
    pub speed: f32,
    pub exp: u32,
    _phantom: PhantomData<S>,
}

impl<S: Side> Enemy<S> {
    pub fn new(health: i32, speed: f32, exp: u32) -> Self {
        Self {
            health,
            speed,
            exp,
            _phantom: PhantomData,
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct EnemyAttack<S: Side> {
    damage: i32,
    range: f32,
    attack_timer: Timer,
    _phantom: PhantomData<S>,
}

impl<S: Side> EnemyAttack<S> {
    pub fn new(damage: i32, range: f32, attack_speed: f32) -> Self {
        // initially timer is paused
        // unpause when in attack range
        let mut attack_timer = Timer::from_seconds(attack_speed, TimerMode::Repeating);
        attack_timer.pause();

        Self {
            damage,
            range,
            attack_timer,
            _phantom: PhantomData,
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct EnemyMarker;

#[derive(Bundle)]
pub struct EnemyBundle<S: Side, E: EnemyType<S>> {
    #[bundle]
    animation_bundle: AnimationBundle,
    rigid_body: RigidBody,
    collider: Collider,
    locked_axis: LockedAxes,
    velocity: Velocity,
    damping: Damping,
    enemy: Enemy<S>,
    attack: EnemyAttack<S>,
    enemy_type: E,
    marker: EnemyMarker,
}

impl<S: Side, E: EnemyType<S>> EnemyBundle<S, E> {
    fn new(
        size: f32,
        texture_atlas: Handle<TextureAtlas>,
        position: Vec3,
        global_buffs: &GlobalEnemyBuffs,
        buffs: &EnemyBuffs<S>,
    ) -> Self {
        Self {
            // Double side for sprites to better correlate with collider size
            animation_bundle: AnimationBundle::new(texture_atlas, size * 2.0, 3, 5.0, position),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(size),
            locked_axis: LockedAxes::ROTATION_LOCKED,
            velocity: Velocity::default(),
            damping: Damping {
                linear_damping: 5.0,
                angular_damping: 10.0,
            },
            enemy: E::enemy(global_buffs, buffs),
            attack: E::attack(global_buffs, buffs),
            enemy_type: E::default(),
            marker: EnemyMarker,
        }
    }
}

pub trait EnemyType<S: Side>: Component + Default {
    const HEALTH: i32;
    const SPEED: f32;
    const EXP: u32;
    const DAMAGE: i32;
    const SIZE: f32;
    // Range should be bigger then enemy size / 2
    const RANGE: f32;
    const ATTACK_SPEED: f32;
    const NUMBER_PER_SPAWN: u32;

    fn enemy(global_buffs: &GlobalEnemyBuffs, buffs: &EnemyBuffs<S>) -> Enemy<S> {
        Enemy::new(
            (Self::HEALTH as f32 * (1.0 + global_buffs.health + buffs.health)) as i32,
            Self::SPEED * (1.0 + global_buffs.speed + buffs.speed),
            (Self::EXP as f32 * (1.0 - (global_buffs.exp + buffs.exp))) as u32,
        )
    }

    fn attack(global_buffs: &GlobalEnemyBuffs, buffs: &EnemyBuffs<S>) -> EnemyAttack<S> {
        EnemyAttack::new(
            (Self::DAMAGE as f32 * (1.0 + global_buffs.damage + buffs.damage)) as i32,
            Self::RANGE,
            Self::ATTACK_SPEED * (1.0 + global_buffs.attack_speed + buffs.attack_speed),
        )
    }

    fn texture_atlas(enemy_sprites: &EnemySprites) -> Handle<TextureAtlas>;
}

#[derive(Debug, Default, Component)]
pub struct MadCrab;

impl<S: Side> EnemyType<S> for MadCrab {
    const HEALTH: i32 = 100;
    const SPEED: f32 = 8.0;
    const EXP: u32 = 3;
    const DAMAGE: i32 = 5;
    const SIZE: f32 = 128.0;
    const RANGE: f32 = 20.0;
    const ATTACK_SPEED: f32 = 1.1;
    const NUMBER_PER_SPAWN: u32 = 1;

    fn texture_atlas(enemy_sprites: &EnemySprites) -> Handle<TextureAtlas> {
        enemy_sprites.mad_crab.clone()
    }
}

#[derive(Debug, Default, Component)]
pub struct Goblin;

impl<S: Side> EnemyType<S> for Goblin {
    const HEALTH: i32 = 80;
    const SPEED: f32 = 15.0;
    const EXP: u32 = 5;
    const DAMAGE: i32 = 10;
    const SIZE: f32 = 16.0;
    const RANGE: f32 = 20.0;
    const ATTACK_SPEED: f32 = 1.0;
    const NUMBER_PER_SPAWN: u32 = 3;

    fn texture_atlas(enemy_sprites: &EnemySprites) -> Handle<TextureAtlas> {
        enemy_sprites.goblin.clone()
    }
}

#[derive(Debug, Default, Component)]
pub struct SpearGoblin;

impl<S: Side> EnemyType<S> for SpearGoblin {
    const HEALTH: i32 = 100;
    const SPEED: f32 = 10.0;
    const EXP: u32 = 8;
    const DAMAGE: i32 = 15;
    const SIZE: f32 = 16.0;
    const RANGE: f32 = 20.0;
    const ATTACK_SPEED: f32 = 1.2;
    const NUMBER_PER_SPAWN: u32 = 2;

    fn texture_atlas(enemy_sprites: &EnemySprites) -> Handle<TextureAtlas> {
        enemy_sprites.spear_goblin.clone()
    }
}

#[derive(Debug, Default, Component)]
pub struct Bat;

impl<S: Side> EnemyType<S> for Bat {
    const HEALTH: i32 = 30;
    const SPEED: f32 = 10.0;
    const EXP: u32 = 5;
    const DAMAGE: i32 = 5;
    const SIZE: f32 = 16.0;
    const RANGE: f32 = 20.0;
    const ATTACK_SPEED: f32 = 1.5;
    const NUMBER_PER_SPAWN: u32 = 5;

    fn texture_atlas(enemy_sprites: &EnemySprites) -> Handle<TextureAtlas> {
        enemy_sprites.bat.clone()
    }
}

#[derive(Debug, Default, Component)]
pub struct Skull;

impl<S: Side> EnemyType<S> for Skull {
    const HEALTH: i32 = 80;
    const SPEED: f32 = 8.0;
    const EXP: u32 = 5;
    const DAMAGE: i32 = 15;
    const SIZE: f32 = 32.0;
    const RANGE: f32 = 20.0;
    const ATTACK_SPEED: f32 = 1.0;
    const NUMBER_PER_SPAWN: u32 = 1;

    fn texture_atlas(enemy_sprites: &EnemySprites) -> Handle<TextureAtlas> {
        enemy_sprites.skull.clone()
    }
}

#[derive(Debug, Default, Component)]
pub struct PoisonIvy;

impl<S: Side> EnemyType<S> for PoisonIvy {
    const HEALTH: i32 = 60;
    const SPEED: f32 = 12.0;
    const EXP: u32 = 8;
    const DAMAGE: i32 = 20;
    const SIZE: f32 = 64.0;
    const RANGE: f32 = 20.0;
    const ATTACK_SPEED: f32 = 1.0;
    const NUMBER_PER_SPAWN: u32 = 2;

    fn texture_atlas(enemy_sprites: &EnemySprites) -> Handle<TextureAtlas> {
        enemy_sprites.poison_ivy.clone()
    }
}

fn setup(mut spawn_state: ResMut<NextState<SpawnState>>, mut commands: Commands) {
    spawn_state.set(SpawnState::Stage1);
    commands.insert_resource(GlobalEnemyBuffs::default());
    commands.insert_resource(SpawnStateTimer::default());
}

fn enemy_spawn_state_progress(
    time: Res<Time>,
    mut spawn_state_timer: ResMut<SpawnStateTimer>,
    mut enemy_state: ResMut<NextState<SpawnState>>,
) {
    if let Some(timer) = &mut spawn_state_timer.timer {
        if timer.tick(time.delta()).finished() {
            spawn_state_timer.next_state();
            enemy_state.set(spawn_state_timer.current_state);
        }
    }
}

/// Moved enemies in direction of the wall
/// Keeps them pointed at the wall
fn enemy_movement<S: Side>(
    time: Res<Time>,
    wall: Query<&Transform, With<CastleWall<S>>>,
    mut enemies: Query<(&Transform, &Enemy<S>, &mut Velocity)>,
) {
    let wall_transform = wall.single();

    for (enemy_transform, enemy, mut enemy_velocity) in enemies.iter_mut() {
        let vector = (wall_transform.translation - enemy_transform.translation).truncate();
        let direction = vector.normalize();

        let movement = direction * time.delta().as_secs_f32();
        enemy_velocity.linvel = movement * enemy.speed * ENEMY_FORCE_MULTIPLIER;
    }
}

fn enemy_attack<S: Side>(
    time: Res<Time>,
    wall: Query<(&Transform, &CastleWall<S>)>,
    mut enemies: Query<(&Transform, &mut EnemyAttack<S>)>,
    mut damage_events: EventWriter<WallDamageEvent<S>>,
) {
    let (wall_transform, wall) = wall.single();

    for (enemy_transform, mut enemy_attack) in enemies.iter_mut() {
        let distance = (wall_transform
            .translation
            .truncate()
            .dot(S::DIRECTION.abs())
            - enemy_transform
                .translation
                .truncate()
                .dot(S::DIRECTION.abs()))
        .abs()
            - wall.half_thickness;

        if enemy_attack.range < distance {
            continue;
        }

        enemy_attack.attack_timer.unpause();

        if !enemy_attack.attack_timer.tick(time.delta()).finished() {
            continue;
        }

        damage_events.send(WallDamageEvent::new(enemy_attack.damage));
    }
}

fn enemy_death<S: Side>(
    enemies: Query<(Entity, &Enemy<S>)>,
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
