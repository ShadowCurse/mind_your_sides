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
