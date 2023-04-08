use bevy::{prelude::*, window::WindowMode};

use super::{spawn_button, UiConfig, UiMainMenuState};
use crate::{utils::remove_all_with, GameSettings};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(UiMainMenuState::Settings)))
            .add_systems(
                (button_system, update_window_mode, update_volume_value)
                    .in_set(OnUpdate(UiMainMenuState::Settings)),
            )
            .add_system(
                remove_all_with::<SettingsMarker>.in_schedule(OnExit(UiMainMenuState::Settings)),
            );
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct SettingsMarker;

#[derive(Debug, Clone, Copy, Component)]
pub struct WindowModeText;

#[derive(Debug, Clone, Copy, Component)]
pub struct VolumeText;

#[derive(Debug, Clone, Copy, Component)]
pub enum SettingsButton {
    FullScreen,
    Windowed,
    VolumeUp,
    VolumeDown,
    Back,
}

fn setup(config: Res<UiConfig>, game_settings: Res<GameSettings>, mut commands: Commands) {
    let _ = spawn_layout(&config, &game_settings, &mut commands);
}

pub fn spawn_layout(
    config: &UiConfig,
    game_settings: &GameSettings,
    commands: &mut Commands,
) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: config.menu_style.clone(),
                background_color: config.panels_background.into(),
                ..default()
            },
            SettingsMarker,
        ))
        .with_children(|builder| {
            // Window mode
            builder
                .spawn((NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: config.panels_background.into(),
                    ..default()
                },))
                .with_children(|builder| {
                    // Current window mode
                    builder.spawn((
                        TextBundle {
                            text: Text::from_section(
                                format!("Window mode: {:?}", game_settings.window_mode),
                                config.text_style.clone(),
                            ),
                            ..default()
                        },
                        WindowModeText,
                    ));

                    // Window mode modes
                    builder
                        .spawn((NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: config.panels_background.into(),
                            ..default()
                        },))
                        .with_children(|builder| {
                            spawn_button(builder, config, SettingsButton::FullScreen);
                            spawn_button(builder, config, SettingsButton::Windowed);
                        });
                });

            // Sound volume
            builder
                .spawn((NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: config.panels_background.into(),
                    ..default()
                },))
                .with_children(|builder| {
                    // Current window mode
                    builder.spawn((
                        TextBundle {
                            text: Text::from_section(
                                format!("Volume: {:.2}", game_settings.sound_volume),
                                config.text_style.clone(),
                            ),
                            ..default()
                        },
                        VolumeText,
                    ));

                    // Volume controls
                    builder
                        .spawn((NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: config.panels_background.into(),
                            ..default()
                        },))
                        .with_children(|builder| {
                            spawn_button(builder, config, SettingsButton::VolumeDown);
                            spawn_button(builder, config, SettingsButton::VolumeUp);
                        });
                });

            spawn_button(builder, config, SettingsButton::Back);
        })
        .id()
}

fn button_system(
    style: Res<UiConfig>,
    mut windows: Query<&mut Window>,
    mut game_settings: ResMut<GameSettings>,
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

pub fn update_window_mode(
    game_settings: Res<GameSettings>,
    mut window_mode_text: Query<&mut Text, With<WindowModeText>>,
) {
    let mut text = window_mode_text.single_mut();
    text.sections[0].value = format!("Window mode: {:?}", game_settings.window_mode);
}

pub fn update_volume_value(
    game_settings: Res<GameSettings>,
    mut volume_text: Query<&mut Text, With<VolumeText>>,
) {
    let mut text = volume_text.single_mut();
    text.sections[0].value = format!("Volume: {:.2}", game_settings.sound_volume);
}
