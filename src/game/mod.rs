use core::fmt::Debug;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_kira_audio::Audio;
use bevy_rapier2d::prelude::RapierConfiguration;

use crate::GameSettings;
use crate::{impl_into_state, utils::set_state, GameAssets, GlobalState, IntoState};

pub mod animation;
pub mod castle;
pub mod damage;
pub mod enemies;
pub mod upgrades;
pub mod weapons;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_system(
                set_state::<GameState, { GameState::InGame as u8 }>
                    .in_schedule(OnEnter(GlobalState::InGame)),
            )
            .add_system(
                set_state::<GameState, { GameState::NotInGame as u8 }>
                    .in_schedule(OnExit(GlobalState::InGame)),
            )
            .add_system(play_audio.in_schedule(OnEnter(GlobalState::InGame)))
            .add_system(stop_audio.in_schedule(OnExit(GlobalState::InGame)))
            .add_system(in_game_key_input.in_set(OnUpdate(GameState::InGame)))
            .add_system(stop_physics.in_schedule(OnEnter(GameState::Paused)))
            .add_system(resume_physics.in_schedule(OnExit(GameState::Paused)))
            .add_system(stop_physics.in_schedule(OnEnter(GameState::GameOver)))
            .add_system(resume_physics.in_schedule(OnExit(GameState::GameOver)))
            .add_system(stop_physics.in_schedule(OnEnter(GameState::LevelUp)))
            .add_system(resume_physics.in_schedule(OnExit(GameState::LevelUp)))
            .add_system(stop_physics.in_schedule(OnEnter(GameState::StatsNorth)))
            .add_system(resume_physics.in_schedule(OnExit(GameState::StatsNorth)))
            .add_system(stop_physics.in_schedule(OnEnter(GameState::StatsSouth)))
            .add_system(resume_physics.in_schedule(OnExit(GameState::StatsSouth)))
            .add_system(stop_physics.in_schedule(OnEnter(GameState::StatsWest)))
            .add_system(resume_physics.in_schedule(OnExit(GameState::StatsWest)))
            .add_system(stop_physics.in_schedule(OnEnter(GameState::StatsEast)))
            .add_system(resume_physics.in_schedule(OnExit(GameState::StatsEast)))
            .add_plugin(animation::AnimationPlugin)
            .add_plugin(castle::CastlePlugin)
            .add_plugin(damage::DamagePlugin)
            .add_plugin(enemies::EnemyPlugin)
            .add_plugin(weapons::WeaponsPlugin)
            .add_plugin(upgrades::UpgradesPlugin);
    }
}

fn stop_physics(mut physics: ResMut<RapierConfiguration>) {
    physics.physics_pipeline_active = false;
}

fn resume_physics(mut physics: ResMut<RapierConfiguration>) {
    physics.physics_pipeline_active = true;
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    NotInGame,
    InGame,
    Paused,
    GameOver,
    LevelUp,
    StatsNorth,
    StatsSouth,
    StatsWest,
    StatsEast,
}
impl_into_state!(GameState);

#[derive(Debug, Default, Clone, Copy)]
pub struct North;

#[derive(Debug, Default, Clone, Copy)]
pub struct South;

#[derive(Debug, Default, Clone, Copy)]
pub struct West;

#[derive(Debug, Default, Clone, Copy)]
pub struct East;

pub trait Side: Debug + Default + Clone + Copy + Send + Sync + 'static {
    const DIRECTION: Vec2;
}

impl Side for North {
    const DIRECTION: Vec2 = Vec2::Y;
}
impl Side for South {
    const DIRECTION: Vec2 = Vec2::NEG_Y;
}
impl Side for West {
    const DIRECTION: Vec2 = Vec2::NEG_X;
}
impl Side for East {
    const DIRECTION: Vec2 = Vec2::X;
}

fn play_audio(audio: Res<Audio>, game_settings: Res<GameSettings>, game_assets: Res<GameAssets>) {
    audio
        .play(game_assets.background.clone())
        .with_volume(game_settings.sound_volume)
        .looped();
}

fn stop_audio(audio: Res<Audio>) {
    audio.stop();
}

fn in_game_key_input(keyboard: Res<Input<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
    if keyboard.pressed(KeyCode::Escape) {
        game_state.set(GameState::Paused);
    }

    if keyboard.pressed(KeyCode::O) {
        game_state.set(GameState::GameOver);
    }

    if keyboard.pressed(KeyCode::L) {
        game_state.set(GameState::LevelUp);
    }
}
