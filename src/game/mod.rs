use bevy::prelude::*;

mod castle;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(castle::CastlePlugin);
    }
}
