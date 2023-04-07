use std::marker::PhantomData;

use bevy::prelude::*;
use rand::prelude::*;

use crate::{game::GameState, utils::remove_all_with, GlobalState};

use super::{
    Bat, EnemyBundle, EnemyMarker, EnemySprites, EnemyType, GlobalEnemyBuffs, Goblin, MadCrab,
    PoisonIvy, Side, Skull, SpawnState, SpearGoblin,
};

const DEFAULT_ENEMY_SPAWN_POSITON: f32 = 1500.0;

const DEFAULT_ENEMY_SPAWN_RADIUS: f32 = 200.0;
const DEFAULT_ENEMY_SPAWN_RATE: f32 = 5.0;

#[derive(Default)]
pub struct SpawnPlugin<S: Side> {
    _phantom: PhantomData<S>,
}

impl<S: Side> Plugin for SpawnPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_system(setup::<S>.in_schedule(OnEnter(GlobalState::InGame)))
            .add_systems(
                (enemy_spawn::<S, Bat>, enemy_spawn::<S, Goblin>)
                    .in_set(OnUpdate(SpawnState::Stage1))
                    .in_set(OnUpdate(GameState::InGame)),
            )
            .add_systems(
                (
                    enemy_spawn::<S, Bat>,
                    enemy_spawn::<S, Goblin>,
                    enemy_spawn::<S, SpearGoblin>,
                    enemy_spawn::<S, Skull>,
                )
                    .in_set(OnUpdate(SpawnState::Stage2))
                    .in_set(OnUpdate(GameState::InGame)),
            )
            .add_systems(
                (
                    enemy_spawn::<S, Bat>,
                    enemy_spawn::<S, Goblin>,
                    enemy_spawn::<S, SpearGoblin>,
                    enemy_spawn::<S, Skull>,
                    enemy_spawn::<S, PoisonIvy>,
                )
                    .in_set(OnUpdate(SpawnState::Stage3))
                    .in_set(OnUpdate(GameState::InGame)),
            )
            .add_systems(
                (
                    enemy_spawn::<S, Bat>,
                    enemy_spawn::<S, Goblin>,
                    enemy_spawn::<S, SpearGoblin>,
                    enemy_spawn::<S, Skull>,
                    enemy_spawn::<S, PoisonIvy>,
                    enemy_spawn::<S, MadCrab>,
                )
                    .in_set(OnUpdate(SpawnState::Stage4))
                    .in_set(OnUpdate(GameState::InGame)),
            )
            .add_system(remove_all_with::<EnemyMarker>.in_schedule(OnEnter(GlobalState::MainMenu)))
            .add_system(
                remove_all_with::<EnemySpawnMarker>.in_schedule(OnEnter(GlobalState::MainMenu)),
            );
    }
}

#[derive(Debug, Default, Component)]
pub struct EnemySpawnMarker;

#[derive(Debug, Default, Resource)]
pub struct EnemyBuffs<S: Side> {
    pub health: f32,
    pub speed: f32,
    pub exp: f32,
    pub damage: f32,
    pub attack_speed: f32,
    _phantom: PhantomData<S>,
}

impl<S: Side> std::fmt::Display for EnemyBuffs<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(" @ health +{:.1}%\n", self.health * 100.0))?;
        f.write_fmt(format_args!(" @ speed +{:.1}%\n", self.speed * 100.0))?;
        f.write_fmt(format_args!(" @ exp -{:.1}%\n", self.exp * 100.0))?;
        f.write_fmt(format_args!(" @ damage +{:.1}%\n", self.damage * 100.0))?;
        f.write_fmt(format_args!(
            " @ attack speed +{:.1}%\n",
            self.attack_speed * 100.0
        ))?;
        Ok(())
    }
}

impl<S: Side> EnemyBuffs<S> {
    pub fn with_global_buffs(&self, global_buffs: &GlobalEnemyBuffs) -> Self {
        Self {
            health: self.health + global_buffs.health,
            speed: self.speed + global_buffs.speed,
            exp: self.exp + global_buffs.exp,
            damage: self.damage + global_buffs.damage,
            attack_speed: self.attack_speed + global_buffs.attack_speed,
            _phantom: PhantomData,
        }
    }
}

#[derive(Debug, Component)]
pub struct EnemySpawn<S: Side, E: EnemyType<S>> {
    pub number: u32,
    pub radius: f32,
    pub timer: Timer,
    _phantom: PhantomData<S>,
    _phantom2: PhantomData<E>,
}

impl<S: Side, E: EnemyType<S>> Default for EnemySpawn<S, E> {
    fn default() -> Self {
        Self {
            number: E::NUMBER_PER_SPAWN,
            radius: DEFAULT_ENEMY_SPAWN_RADIUS,
            timer: Timer::from_seconds(DEFAULT_ENEMY_SPAWN_RATE, TimerMode::Repeating),
            _phantom: PhantomData,
            _phantom2: PhantomData,
        }
    }
}

#[derive(Default, Bundle)]
pub struct EnemySpawnBundle<S: Side, E: EnemyType<S>> {
    spawn: EnemySpawn<S, E>,
    marker: EnemySpawnMarker,
}

/// Sets up 4 spawns at each side of the screen
fn setup<S: Side>(mut commands: Commands) {
    commands.insert_resource(EnemyBuffs::<S>::default());

    // North
    commands
        .spawn(TransformBundle::from_transform(
            Transform::from_translation((S::DIRECTION * DEFAULT_ENEMY_SPAWN_POSITON).extend(0.0)),
        ))
        .insert(EnemySpawnBundle::<S, Bat>::default())
        .insert(EnemySpawnBundle::<S, Goblin>::default())
        .insert(EnemySpawnBundle::<S, SpearGoblin>::default())
        .insert(EnemySpawnBundle::<S, Skull>::default())
        .insert(EnemySpawnBundle::<S, PoisonIvy>::default())
        .insert(EnemySpawnBundle::<S, MadCrab>::default());
}

/// Spawns enemies in a circle arond the spawn point equally spread
/// on a circle
fn enemy_spawn<S: Side, E: EnemyType<S>>(
    time: Res<Time>,
    enemy_sprites: Res<EnemySprites>,
    global_buffs: Res<GlobalEnemyBuffs>,
    buffs: Res<EnemyBuffs<S>>,
    mut commands: Commands,
    mut spawns: Query<(&Transform, &mut EnemySpawn<S, E>)>,
) {
    for (transform, mut spawn) in spawns.iter_mut() {
        if !spawn.timer.tick(time.delta()).finished() {
            continue;
        }

        for n in 0..spawn.number {
            let position = transform.translation
                + Quat::from_rotation_z(
                    (2.0 * std::f32::consts::PI / spawn.number as f32) * n as f32
                        + rand::thread_rng().gen_range(0.0..std::f32::consts::FRAC_PI_6),
                )
                .mul_vec3(Vec3::Y * spawn.radius);

            commands.spawn(EnemyBundle::<S, E>::new(
                E::SIZE,
                E::texture_atlas(&enemy_sprites),
                position,
                &global_buffs,
                &buffs,
            ));
        }
    }
}
