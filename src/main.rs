#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

mod game;
mod ui;
mod utils;

use utils::IntoState;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::GRAY))
        .add_state::<GlobalState>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(WorldInspectorPlugin::new())// Just for Debug
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // debug 2d physics
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(game::GamePlugin)
        .add_plugin(ui::UiPlugin)
        .add_startup_system(setup)
        .run();
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, States)]
pub enum GlobalState {
    #[default]
    MainMenu,
    InGame,
}
impl_into_state!(GlobalState);

fn setup(mut commands: Commands, mut physics: ResMut<RapierConfiguration>) {
    // disable gravity because game is 2d top down
    physics.gravity = Vec2::ZERO;

    let mut camera_bundle = Camera2dBundle::default();
    // make everything smaller
    camera_bundle.projection.scale = 1.5;
    commands.spawn(camera_bundle);
}
