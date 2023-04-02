use bevy::prelude::*;

mod castle;
mod enemies;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(castle::CastlePlugin)
            .add_plugin(enemies::EnemyPlugin);
    }
}
