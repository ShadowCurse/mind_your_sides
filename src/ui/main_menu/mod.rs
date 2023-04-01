use bevy::prelude::*;

use crate::{
    impl_into_state,
    utils::{set_state, IntoState},
    GlobalState,
};

use super::{spawn_button, UiConfig};

mod settings;
mod title_screen;

pub struct UiMainMenuPlugin;

impl Plugin for UiMainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<UiMainMenuState>()
            .add_system(
                set_state::<UiMainMenuState, { UiMainMenuState::TitleScreen as u8 }>
                    .in_schedule(OnEnter(GlobalState::MainMenu)),
            )
            .add_system(
                set_state::<UiMainMenuState, { UiMainMenuState::Disabled as u8 }>
                    .in_schedule(OnEnter(GlobalState::InGame)),
            )
            .add_plugin(settings::SettingsPlugin)
            .add_plugin(title_screen::TitleScreenPlugin);
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, States)]
enum UiMainMenuState {
    #[default]
    Disabled,
    TitleScreen,
    Settings,
}
impl_into_state!(UiMainMenuState);
