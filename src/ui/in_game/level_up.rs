use crate::game::GameState;
use crate::ui::in_game::hud::LevelUpEvent;
use crate::ui::in_game::UiInGameState;
use crate::ui::UiConfig;
use crate::utils::remove_all_with;

use bevy::prelude::*;

pub struct LevelUpPlugin;

impl Plugin for LevelUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(level_up_event_reader)
            .add_system(setup.in_schedule(OnEnter(UiInGameState::LevelUp)))
            .add_system(button_system.in_set(OnUpdate(UiInGameState::LevelUp)))
            .add_system(
                remove_all_with::<LevelUpMarker>.in_schedule(OnExit(UiInGameState::LevelUp)),
            );
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, Component)]
enum GameCards {
    WallNorth,
    WallSouth,
    WallWest,
    WallEast,
}

#[derive(Debug, Clone, Copy, Component)]
struct LevelUpMarker;

fn setup(mut commands: Commands, config: Res<UiConfig>) {
    commands
        .spawn((
            NodeBundle {
                style: config.menu_style.clone(),
                background_color: config.menu_color.into(),
                ..default()
            },
            LevelUpMarker,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Buffs",
                config.title_text_style.clone(),
            ));
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        margin: UiRect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: config.menu_color.into(),
                    ..default()
                })
                .with_children(|builder| {
                    spawn_card(builder, &config, GameCards::WallNorth);
                    spawn_card(builder, &config, GameCards::WallSouth);
                    spawn_card(builder, &config, GameCards::WallWest);
                    spawn_card(builder, &config, GameCards::WallEast);
                });
        });
}

fn level_up_event_reader(
    mut game_state: ResMut<NextState<GameState>>,
    mut event: EventReader<LevelUpEvent>,
) {
    for _ev in event.iter() {
        println!("Level UP");
        game_state.set(GameState::LevelUp);
    }
}

fn button_system(
    style: Res<UiConfig>,
    mut game_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&GameCards, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = style.button_color_pressed.into();
                match button {
                    GameCards::WallNorth => {
                        game_state.set(GameState::InGame);
                    }
                    GameCards::WallSouth => {
                        game_state.set(GameState::InGame);
                    }
                    GameCards::WallWest => {
                        game_state.set(GameState::InGame);
                    }
                    GameCards::WallEast => {
                        game_state.set(GameState::InGame);
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

fn spawn_card<B>(child_builder: &mut ChildBuilder, style: &UiConfig, button: B)
where
    B: Component + std::fmt::Debug + Copy,
{
    child_builder
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(250.0), Val::Px(300.0)),
                    margin: UiRect::all(Val::Percent(1.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: style.button_color_normal.into(),
                ..default()
            },
            button,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(format!("{button:?}"), style.text_style.clone()),
                ..default()
            });
        });
}
