use bevy::prelude::*;

use crate::{
    ui::{spawn_button, UiConfig},
    utils::remove_all_with, game::GameState,
};

use super::UiInGameState;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(UiInGameState::InGame)))
            .add_system(button_system.in_set(OnUpdate(UiInGameState::InGame)))
            .add_system(remove_all_with::<HUDMarker>.in_schedule(OnExit(UiInGameState::InGame)));
    }
}

#[derive(Debug, Clone, Copy, Component)]
struct HUDMarker;

#[derive(Debug, Clone, Copy, Component)]
enum HUDButton {
    Pause,
}

fn setup(mut commands: Commands, config: Res<UiConfig>) {
    // root node
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::width(Val::Percent(100.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            HUDMarker,
        ))
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::width(Val::Px(200.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::width(Val::Percent(100.0)),
                                ..default()
                            },
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // left vertical fill (content)
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        size: Size::width(Val::Percent(100.0)),
                                        ..default()
                                    },
                                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    spawn_button(parent, &config, HUDButton::Pause);
                                    // text
                                    parent.spawn((TextBundle::from_section(
                                        "Text on the left",
                                        config.text_style.clone(),
                                    )
                                    .with_style(Style {
                                        margin: UiRect::all(Val::Px(5.0)),
                                        ..default()
                                    }),));
                                });
                        });
                });
            // right vertical fill
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::width(Val::Px(200.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn((TextBundle::from_section(
                        "Text on the right",
                        config.text_style.clone(),
                    )
                    .with_style(Style {
                        size: Size::height(Val::Px(25.)),
                        ..default()
                    }),));
                });
        });
}

fn button_system(
    style: Res<UiConfig>,
    mut game_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&HUDButton, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = style.button_color_pressed.into();
                match button {
                    HUDButton::Pause => {
                        game_state.set(GameState::Paused);
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
