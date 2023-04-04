use core::fmt::Debug;

use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierConfiguration;

use crate::{impl_into_state, utils::set_state, GlobalState, IntoState};

pub mod animation;
pub mod castle;
pub mod damage;
pub mod enemies;
pub mod weapons;
pub mod upgrades;

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
            .add_system(stop_physics.in_schedule(OnEnter(GameState::Paused)))
            .add_system(resume_physics.in_schedule(OnExit(GameState::Paused)))
            .add_system(stop_physics.in_schedule(OnEnter(GameState::LevelUp)))
            .add_system(resume_physics.in_schedule(OnExit(GameState::LevelUp)))
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
}
impl_into_state!(GameState);

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct North;

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct South;

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct West;

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct East;

pub trait Side: Debug + Default + Clone + Copy + Component {
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
