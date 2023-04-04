use bevy::prelude::*;

use crate::{
    game::castle::Castle,
    game::GameState,
    ui::{spawn_button, UiConfig},
    utils::remove_all_with,
};

use super::UiInGameState;

pub struct HUDPlugin;

const MAX_EXP: u32 = 600;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelUpEvent>()
            .add_system(setup.in_schedule(OnEnter(UiInGameState::InGame)))
            .add_system(update_text_level.in_set(OnUpdate(UiInGameState::InGame)))
            .add_system(button_system.in_set(OnUpdate(UiInGameState::InGame)))
            .add_system(remove_all_with::<HUDMarker>.in_schedule(OnExit(UiInGameState::InGame)));
    }
}

#[derive(Debug, Clone, Copy, Component)]
struct HUDMarker;
#[derive(Debug, Clone, Copy, Component)]
struct LevelText;

pub struct LevelUpEvent();

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
                    // text
                    parent.spawn(TextBundle::from_section(
                        "Text on the left",
                        config.text_style.clone(),
                    ));
                    parent.spawn(TextBundle::from_section(
                        "Exp : ",
                        config.text_style.clone(),
                    ));
                    parent.spawn((
                        TextBundle::from_section("0", config.text_style.clone()),
                        LevelText,
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

fn update_text_level(
    mut event: EventWriter<LevelUpEvent>,
    mut level_text: Query<&mut Text, With<LevelText>>,
    mut castle: Query<&mut Castle>,
) {
    for mut text in &mut level_text {
        let mut castle = castle.single_mut();
        if castle.exp > MAX_EXP {
            castle.exp = 0;
            event.send(LevelUpEvent())
        }
        text.sections[0].value = format!("{}", castle.exp);
    }
}
