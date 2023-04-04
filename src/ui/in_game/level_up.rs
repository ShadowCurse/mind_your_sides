use bevy::prelude::*;

pub struct LevelUpPlugin;

impl Plugin for LevelUpPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Debug, Clone, Copy, Component)]
struct LevelUpMarker;

fn setup(){}