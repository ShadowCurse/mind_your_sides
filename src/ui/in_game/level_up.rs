use bevy::prelude::*;
use crate::ui::in_game::hud::LevelUpEvent;

pub struct LevelUpPlugin;

impl Plugin for LevelUpPlugin {
    fn build(&self, app: &mut App) {
        app .add_system(level_up_event_reader);
    }
}

#[derive(Debug, Clone, Copy, Component)]
struct LevelUpMarker;

fn setup(){}


fn level_up_event_reader(
    mut event : EventReader<LevelUpEvent>
){
    for ev in event.iter() {
        println!("1 Level UP ")
    }
}