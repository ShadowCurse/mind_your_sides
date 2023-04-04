#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::{prelude::*, window::PresentMode};
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

mod game;
mod ui;
mod utils;

use utils::IntoState;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::rgb_u8(27, 62, 60)))
        .add_state::<GlobalState>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // asests
        .add_loading_state(
            LoadingState::new(GlobalState::AssetLoading)
                .continue_to_state(GlobalState::Initialization),
        )
        .add_collection_to_loading_state::<_, GameAssets>(GlobalState::AssetLoading)
        // debug
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        //
        .add_plugin(game::GamePlugin)
        .add_plugin(ui::UiPlugin)
        .add_system(setup.in_set(OnUpdate(GlobalState::Initialization)));
    app.run();
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, States)]
pub enum GlobalState {
    #[default]
    AssetLoading,
    /// Init resources that depend on assets
    /// i.e UI resources
    Initialization,
    MainMenu,
    InGame,
}
impl_into_state!(GlobalState);

#[derive(AssetCollection, Resource)]
struct GameAssets {
    #[asset(path = "fonts/MinimalPixel.ttf")]
    font: Handle<Font>,
}

/// Used to create initial global config
/// and then changes state to `GlobalState::MainMenu`
fn setup(
    mut commands: Commands,
    mut physics: ResMut<RapierConfiguration>,
    mut windows: Query<&mut Window>,
    mut global_state: ResMut<NextState<GlobalState>>,
) {
    // disable gravity because game is 2d top down
    physics.gravity = Vec2::ZERO;

    let mut camera_bundle = Camera2dBundle::default();
    // make everything smaller
    camera_bundle.projection.scale = 1.0;
    commands.spawn(camera_bundle);

    for mut window in windows.iter_mut() {
        window.present_mode = PresentMode::AutoVsync;
    }

    global_state.set(GlobalState::MainMenu);
}
