use bevy::{prelude::*, window::WindowMode};

use crate::{
    ui::{in_game::hud::HUDMarker, main_menu::settings::*, UiConfig},
    utils::remove_all_with,
    GameSettings,
};

use super::UiPauseState;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(UiPauseState::Settings)))
            .add_systems(
                (button_system, update_window_mode, update_volume_value)
                    .in_set(OnUpdate(UiPauseState::Settings)),
            )
            .add_system(
                remove_all_with::<SettingsMarker>.in_schedule(OnExit(UiPauseState::Settings)),
            );
    }
}

fn setup(
    config: Res<UiConfig>,
    game_settings: Res<GameSettings>,
    hud: Query<Entity, With<HUDMarker>>,
    mut commands: Commands,
) {
    let settings = spawn_layout(&config, &game_settings, &mut commands);
    let hud = hud.single();
    commands.entity(hud).insert_children(1, &[settings]);
}

fn button_system(
    style: Res<UiConfig>,
    mut windows: Query<&mut Window>,
    mut game_settings: ResMut<GameSettings>,
    mut pause_state: ResMut<NextState<UiPauseState>>,
    mut interaction_query: Query<
        (&SettingsButton, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = style.button_color_pressed.into();
                match button {
                    SettingsButton::FullScreen => {
                        game_settings.window_mode = WindowMode::Fullscreen;
                        windows.single_mut().mode = WindowMode::Fullscreen;
                    }
                    SettingsButton::Windowed => {
                        game_settings.window_mode = WindowMode::Windowed;
                        windows.single_mut().mode = WindowMode::Windowed;
                    }
                    SettingsButton::VolumeUp => {
                        game_settings.sound_volume += 0.05;
                    }
                    SettingsButton::VolumeDown => {
                        game_settings.sound_volume -= 0.05;
                    }
                    SettingsButton::Back => {
                        pause_state.set(UiPauseState::Pause);
                    }
                }
            }
            Interaction::Hovered => {
                *color = style.button_color_hover.into();
            }
            Interaction::None => {
                *color = style.button_color_normal.into();
            }
        }
    }
}
