use std::{marker::PhantomData, time::Duration};

use bevy::prelude::*;

use crate::{
    game::{castle::Castle, Side},
    game::{castle::CastleWall, East, GameState, North, South, West},
    ui::{spawn_button, UiConfig},
    utils::remove_all_with,
    GlobalState,
};

use super::UiInGameState;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GlobalState::InGame)))
            .add_systems(
                (
                    update_time,
                    update_castle_level,
                    update_castle_exp,
                    update_castle_wall_hp::<North>,
                    update_castle_wall_hp::<South>,
                    update_castle_wall_hp::<West>,
                    update_castle_wall_hp::<East>,
                )
                    .in_set(OnUpdate(UiInGameState::InGame)),
            )
            .add_system(button_system.in_set(OnUpdate(UiInGameState::InGame)))
            .add_system(remove_all_with::<HUDMarker>.in_schedule(OnExit(GlobalState::InGame)));
    }
}

#[derive(Resource)]
pub struct HUDTimer {
    start_time: Duration,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct HUDMarker;

#[derive(Debug, Clone, Copy, Component)]
struct TimeText;

#[derive(Debug, Clone, Copy, Component)]
struct CastleLevelText;

#[derive(Debug, Clone, Copy, Component)]
struct CastleExpText;

#[derive(Debug, Default, Clone, Copy, Component)]
struct CastleWallHpText<S: Side> {
    _phantom: PhantomData<S>,
}

#[derive(Debug, Clone, Copy, Component)]
enum HUDButton {
    StatsNorth,
    StatsSouth,
    StatsWest,
    StatsEast,
    Pause,
}

fn setup(time: Res<Time>, config: Res<UiConfig>, mut commands: Commands) {
    commands.insert_resource(HUDTimer {
        start_time: time.elapsed(),
    });
    // root node
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::all(Val::Percent(100.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
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
                        size: Size {
                            width: Val::Percent(3.5 / 16.0 * 100.0),
                            // width: Val::Percent(20.0),
                            height: Val::Percent(100.0),
                        },
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: config.panels_background.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section("Time: ", config.text_style.clone()),
                        TimeText,
                    ));
                    // Castle info
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: config.panels_background.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section("Level: ", config.text_style.clone()),
                                CastleLevelText,
                            ));
                            parent.spawn((
                                TextBundle::from_section("Exp: ", config.text_style.clone()),
                                CastleExpText,
                            ));
                        });

                    // North info
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: config.panels_background.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((TextBundle::from_section(
                                "North",
                                config.text_style.clone(),
                            ),));
                            parent.spawn((
                                TextBundle::from_section("Hp: ", config.text_style.clone()),
                                CastleWallHpText::<North>::default(),
                            ));
                            spawn_button(parent, &config, HUDButton::StatsNorth);
                        });

                    // South info
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: config.panels_background.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((TextBundle::from_section(
                                "South",
                                config.text_style.clone(),
                            ),));
                            parent.spawn((
                                TextBundle::from_section("Hp: ", config.text_style.clone()),
                                CastleWallHpText::<South>::default(),
                            ));
                            spawn_button(parent, &config, HUDButton::StatsSouth);
                        });
                });
            // right vertical fill
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size {
                            width: Val::Percent(3.5 / 16.0 * 100.0),
                            height: (Val::Percent(100.0)),
                        },
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: config.panels_background.into(),
                    ..default()
                })
                .with_children(|parent| {
                    spawn_button(parent, &config, HUDButton::Pause);

                    // West info
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: config.panels_background.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((TextBundle::from_section(
                                "West",
                                config.text_style.clone(),
                            ),));
                            parent.spawn((
                                TextBundle::from_section("Hp: ", config.text_style.clone()),
                                CastleWallHpText::<West>::default(),
                            ));
                            spawn_button(parent, &config, HUDButton::StatsWest);
                        });

                    // East info
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: config.panels_background.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((TextBundle::from_section(
                                "East",
                                config.text_style.clone(),
                            ),));
                            parent.spawn((
                                TextBundle::from_section("Hp: ", config.text_style.clone()),
                                CastleWallHpText::<East>::default(),
                            ));
                            spawn_button(parent, &config, HUDButton::StatsEast);
                        });
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
                    HUDButton::Pause => game_state.set(GameState::Paused),
                    HUDButton::StatsNorth => game_state.set(GameState::StatsNorth),
                    HUDButton::StatsSouth => game_state.set(GameState::StatsSouth),
                    HUDButton::StatsWest => game_state.set(GameState::StatsWest),
                    HUDButton::StatsEast => game_state.set(GameState::StatsEast),
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

fn update_time(
    time: Res<Time>,
    hud_timer: Res<HUDTimer>,
    mut time_text: Query<&mut Text, With<TimeText>>,
) {
    let mut text = time_text.single_mut();
    text.sections[0].value = format!(
        "Time: {:.1}",
        (time.elapsed() - hud_timer.start_time).as_secs_f32()
    );
}

fn update_castle_level(
    castle: Query<&Castle>,
    mut level_text: Query<&mut Text, With<CastleLevelText>>,
) {
    let castle = castle.single();
    let mut level_text = level_text.single_mut();
    level_text.sections[0].value = format!("Level: {}", castle.level);
}

fn update_castle_exp(castle: Query<&Castle>, mut exp_text: Query<&mut Text, With<CastleExpText>>) {
    let castle = castle.single();
    let mut exp_text = exp_text.single_mut();
    exp_text.sections[0].value = format!("Exp: {}/{}", castle.exp, castle.next_level_exp);
}

fn update_castle_wall_hp<S: Side>(
    wall: Query<&CastleWall<S>>,
    mut hp_text: Query<&mut Text, With<CastleWallHpText<S>>>,
) {
    let wall = wall.single();
    let mut hp_text = hp_text.single_mut();
    hp_text.sections[0].value = format!("Hp: {}/{}", wall.health, wall.max_health);
}
