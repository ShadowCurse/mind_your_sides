use bevy::prelude::*;

use super::{spawn_button, UiConfig, UiMainMenuState};
use crate::utils::remove_all_with;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(UiMainMenuState::Settings)))
            .add_system(button_system.in_set(OnUpdate(UiMainMenuState::Settings)))
            .add_system(
                remove_all_with::<SettingsMarker>.in_schedule(OnExit(UiMainMenuState::Settings)),
            );
    }
}

#[derive(Debug, Clone, Copy, Component)]
struct SettingsMarker;

#[derive(Debug, Clone, Copy, Component)]
enum SettingsButton {
    FullScreen,
    Windowed,
    VolumeUp,
    VolumeDown,
    Back,
}

fn setup(mut commands: Commands, config: Res<UiConfig>) {
    commands
        .spawn((
            NodeBundle {
                style: config.menu_style.clone(),
                background_color: config.menu_color.into(),
                ..default()
            },
            SettingsMarker,
        ))
        .with_children(|builder| {
            spawn_button(builder, &config, SettingsButton::FullScreen);
            spawn_button(builder, &config, SettingsButton::Windowed);
            spawn_button(builder, &config, SettingsButton::VolumeUp);
            spawn_button(builder, &config, SettingsButton::VolumeDown);
            spawn_button(builder, &config, SettingsButton::Back);
        });
}

fn button_system(
    style: Res<UiConfig>,
    mut main_menu_state: ResMut<NextState<UiMainMenuState>>,
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
                    SettingsButton::FullScreen => {}
                    SettingsButton::Windowed => {}
                    SettingsButton::VolumeUp => {}
                    SettingsButton::VolumeDown => {}
                    SettingsButton::Back => {
                        main_menu_state.set(UiMainMenuState::TitleScreen);
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
