mod  level_up;
mod player;
use bevy::prelude::*;
use crate::ui::in_game::player::PlayerUIPlugin;


pub struct InGamePlugin;

impl Plugin for InGamePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerUIPlugin);
    }
}