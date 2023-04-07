use std::marker::PhantomData;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::*;

use crate::{game::GameState, utils::remove_all_with, GlobalState};

use super::{
    Bat, East, EnemyBundle, EnemyMarker, EnemySprites, GlobalEnemyBuffs, Goblin, MadCrab, North,
    PoisonIvy, Side, Skull, South, SpearGoblin, West,
};

const DEFAULT_ENEMY_SIZE: f32 = 16.0;

const DEFAULT_ENEMY_SPAWN_POSITON: f32 = 1000.0;

const DEFAULT_ENEMY_SPAWN_NUMBER: u32 = 2;
const DEFAULT_ENEMY_SPAWN_RADIUS: f32 = 150.0;
const DEFAULT_ENEMY_SPAWN_RATE: f32 = 5.0;

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GlobalState::InGame)))
            .add_systems(
                (
                    enemy_spawn::<North>,
                    enemy_spawn::<South>,
                    enemy_spawn::<West>,
                    enemy_spawn::<East>,
                )
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
pub struct EnemySpawn<S: Side> {
    pub number: u32,
    pub radius: f32,
    pub timer: Timer,
    _phantom: PhantomData<S>,
}

impl<S: Side> Default for EnemySpawn<S> {
    fn default() -> Self {
        Self {
            number: DEFAULT_ENEMY_SPAWN_NUMBER,
            radius: DEFAULT_ENEMY_SPAWN_RADIUS,
            timer: Timer::from_seconds(DEFAULT_ENEMY_SPAWN_RATE, TimerMode::Repeating),
            _phantom: PhantomData,
        }
    }
}

#[derive(Default, Bundle)]
pub struct EnemySpawnBundle<S: Side> {
    spawn: EnemySpawn<S>,
    marker: EnemySpawnMarker,
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

    commands.insert_resource(EnemyBuffs::<North>::default());
    commands.insert_resource(EnemyBuffs::<South>::default());
    commands.insert_resource(EnemyBuffs::<West>::default());
    commands.insert_resource(EnemyBuffs::<East>::default());

    // North
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: spawn_mesh.clone().into(),
            material: spawn_material.clone(),
            transform: Transform::from_translation(Vec3::new(
                0.0,
                DEFAULT_ENEMY_SPAWN_POSITON,
                0.0,
            )),
            ..default()
        })
        .insert(EnemySpawnBundle::<North>::default());
    // South
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: spawn_mesh.clone().into(),
            material: spawn_material.clone(),
            transform: Transform::from_translation(Vec3::new(
                0.0,
                -DEFAULT_ENEMY_SPAWN_POSITON,
                0.0,
            )),
            ..default()
        })
        .insert(EnemySpawnBundle::<South>::default());
    // West
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: spawn_mesh.clone().into(),
            material: spawn_material.clone(),
            transform: Transform::from_translation(Vec3::new(
                -DEFAULT_ENEMY_SPAWN_POSITON,
                0.0,
                0.0,
            )),
            ..default()
        })
        .insert(EnemySpawnBundle::<West>::default());
    // East
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: spawn_mesh.into(),
            material: spawn_material,
            transform: Transform::from_translation(Vec3::new(
                DEFAULT_ENEMY_SPAWN_POSITON,
                0.0,
                0.0,
            )),
            ..default()
        })
        .insert(EnemySpawnBundle::<East>::default());
}

/// Spawns enemies in a circle arond the spawn point equally spread
/// on a circle
fn enemy_spawn<S: Side>(
    time: Res<Time>,
    enemy_sprites: Res<EnemySprites>,
    global_buffs: Res<GlobalEnemyBuffs>,
    buffs: Res<EnemyBuffs<S>>,
    mut commands: Commands,
    mut spawns: Query<(&Transform, &mut EnemySpawn<S>)>,
) {
    for (transform, mut spawn) in spawns.iter_mut() {
        if !spawn.timer.tick(time.delta()).finished() {
            continue;
        }

        let mut rng = rand::thread_rng();

        for n in 0..spawn.number {
            let position = transform.translation
                + Quat::from_rotation_z(
                    (2.0 * std::f32::consts::PI / spawn.number as f32) * n as f32
                        + rand::thread_rng().gen_range(0.0..std::f32::consts::FRAC_PI_6),
                )
                .mul_vec3(Vec3::Y * spawn.radius);

            // Choose enemy at random for now
            match rng.gen_range(0..6) {
                0 => commands.spawn(EnemyBundle::<S, MadCrab>::new(
                    DEFAULT_ENEMY_SIZE,
                    enemy_sprites.mad_crab.clone(),
                    position,
                    &global_buffs,
                    &buffs,
                )),
                1 => commands.spawn(EnemyBundle::<S, Goblin>::new(
                    DEFAULT_ENEMY_SIZE,
                    enemy_sprites.goblin.clone(),
                    position,
                    &global_buffs,
                    &buffs,
                )),
                2 => commands.spawn(EnemyBundle::<S, SpearGoblin>::new(
                    DEFAULT_ENEMY_SIZE,
                    enemy_sprites.spear_goblin.clone(),
                    position,
                    &global_buffs,
                    &buffs,
                )),
                3 => commands.spawn(EnemyBundle::<S, Bat>::new(
                    DEFAULT_ENEMY_SIZE,
                    enemy_sprites.bat.clone(),
                    position,
                    &global_buffs,
                    &buffs,
                )),
                4 => commands.spawn(EnemyBundle::<S, Skull>::new(
                    DEFAULT_ENEMY_SIZE,
                    enemy_sprites.skull.clone(),
                    position,
                    &global_buffs,
                    &buffs,
                )),
                _ => commands.spawn(EnemyBundle::<S, PoisonIvy>::new(
                    DEFAULT_ENEMY_SIZE,
                    enemy_sprites.poison_ivy.clone(),
                    position,
                    &global_buffs,
                    &buffs,
                )),
            };
        }
    }
}
