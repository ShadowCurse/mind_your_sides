use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{
    game::{castle::Castle, Side},
    game::{castle::CastleWall, East, GameState, North, South, West},
    ui::{spawn_button, UiConfig},
    utils::remove_all_with,
};

use super::UiInGameState;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(UiInGameState::InGame)))
            .add_systems(
                (
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
            .add_system(remove_all_with::<HUDMarker>.in_schedule(OnExit(UiInGameState::InGame)));
    }
}

#[derive(Debug, Clone, Copy, Component)]
struct HUDMarker;

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
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    spawn_button(parent, &config, HUDButton::Pause);
                    parent.spawn((
                        TextBundle::from_section("Level: ", config.text_style.clone()),
                        CastleLevelText,
                    ));
                    parent.spawn((
                        TextBundle::from_section("Exp: ", config.text_style.clone()),
                        CastleExpText,
                    ));
                    parent.spawn((
                        TextBundle::from_section("North wall hp: ", config.text_style.clone()),
                        CastleWallHpText::<North>::default(),
                    ));
                    parent.spawn((
                        TextBundle::from_section("South wall hp: ", config.text_style.clone()),
                        CastleWallHpText::<South>::default(),
                    ));
                    parent.spawn((
                        TextBundle::from_section("West wall hp: ", config.text_style.clone()),
                        CastleWallHpText::<West>::default(),
                    ));
                    parent.spawn((
                        TextBundle::from_section("East wall hp: ", config.text_style.clone()),
                        CastleWallHpText::<East>::default(),
                    ));
                });
            // right vertical fill
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::width(Val::Px(200.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(TextBundle::from_section(
                        "Text on the right",
                        config.text_style.clone(),
                    ));
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
    exp_text.sections[0].value = format!("Exp: {}", castle.exp);
}

fn update_castle_wall_hp<S: Side>(
    wall: Query<&CastleWall<S>>,
    mut hp_text: Query<&mut Text, With<CastleWallHpText<S>>>,
) {
    let wall = wall.single();
    let mut hp_text = hp_text.single_mut();
    hp_text.sections[0].value = format!(
        "{:?} wall: {}/{}",
        S::default(),
        wall.health,
        wall.max_health
    );
}
