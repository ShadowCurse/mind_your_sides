use core::fmt::Debug;

use bevy::prelude::*;

mod castle;
mod enemies;
mod weapons;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(castle::CastlePlugin)
            .add_plugin(enemies::EnemyPlugin)
            .add_plugin(weapons::WeaponsPlugin);
    }
}

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct North;

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct South;

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct West;

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct East;

pub trait Side: Debug + Default + Clone + Copy + Component {}

impl Side for North {}
impl Side for South {}
impl Side for West {}
impl Side for East {}
