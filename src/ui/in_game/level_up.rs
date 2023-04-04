use std::thread::spawn;
use bevy::prelude::*;
use crate::game::GameState;
use crate::GlobalState;
use crate::ui::in_game::hud::LevelUpEvent;
use crate::ui::{spawn_button, UiConfig};
use crate::ui::in_game::UiInGameState;
use crate::utils::remove_all_with;

pub struct LevelUpPlugin;

impl Plugin for LevelUpPlugin {
    fn build(&self, app: &mut App) {
        app .add_system(level_up_event_reader)
            .add_system(setup.in_schedule(OnEnter(UiInGameState::LevelUp)))
            .add_system(button_system.in_set(OnUpdate(UiInGameState::LevelUp)))
            .add_system(remove_all_with::<LevelUpMarker>.in_schedule(OnExit(UiInGameState::LevelUp)));
    }
}

#[derive(Debug, Clone, Copy, Component)]
enum GameCards {
    WallNorth,
    WallSouth,
    WallWest,
    WallEast,
}

#[derive(Debug, Clone, Copy, Component)]
struct LevelUpMarker;

fn setup(mut commands: Commands, config: Res<UiConfig>){
    commands
        .spawn((
            NodeBundle {
                style: config.menu_style.clone(),
                background_color: config.menu_color.into(),
                ..default()
            },
            LevelUpMarker,
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                "Buffs",
                config.text_style.clone(),
            ));
            spawn_button(builder, &config, GameCards::WallNorth);
            spawn_button(builder, &config, GameCards::WallSouth);
            spawn_button(builder, &config, GameCards::WallWest);
            spawn_button(builder, &config, GameCards::WallEast);
        });
}


fn level_up_event_reader(
    mut game_state: ResMut<NextState<GameState>>,
    mut event : EventReader<LevelUpEvent>
){
    for ev in event.iter() {
        println!("Level UP");
        game_state.set(GameState::LevelUp);
    }
}

fn button_system(
    style: Res<UiConfig>,
    mut global_state: ResMut<NextState<GlobalState>>,
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