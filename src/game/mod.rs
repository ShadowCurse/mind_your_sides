use core::fmt::Debug;

use bevy::prelude::*;

mod animation;
mod castle;
mod damage;
mod enemies;
mod weapons;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(animation::AnimationPlugin)
            .add_plugin(castle::CastlePlugin)
            .add_plugin(damage::DamagePlugin)
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
