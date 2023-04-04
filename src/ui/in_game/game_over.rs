use bevy::prelude::*;

use crate::{
    ui::{spawn_button, UiConfig},
    utils::remove_all_with,
    GlobalState,
};

use super::UiInGameState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(UiInGameState::GameOver)))
            .add_system(button_system.in_set(OnUpdate(UiInGameState::GameOver)))
            .add_system(
                remove_all_with::<GameOverMarker>.in_schedule(OnExit(UiInGameState::GameOver)),
            );
    }
}

#[derive(Debug, Clone, Copy, Component)]
struct GameOverMarker;

#[derive(Debug, Clone, Copy, Component)]
enum GameOverButton {
    Restart,
    Settings,
    MainMenu,
}

fn setup(mut commands: Commands, config: Res<UiConfig>) {
    commands
        .spawn((
            NodeBundle {
                style: config.menu_style.clone(),
                background_color: config.menu_color.into(),
                ..default()
            },
            GameOverMarker,
        ))
        .with_children(|builder| {
            builder.spawn(
                (TextBundle {
                    text: Text::from_section("Game Over", config.title_text_style.clone()),
                    ..default()
                })
                .with_style(config.title_style.clone()),
            );
        })
        .with_children(|builder| {
            spawn_button(builder, &config, GameOverButton::Restart);
            spawn_button(builder, &config, GameOverButton::Settings);
            spawn_button(builder, &config, GameOverButton::MainMenu);
        });
}

fn button_system(
    style: Res<UiConfig>,
    mut global_state: ResMut<NextState<GlobalState>>,
    mut interaction_query: Query<
        (&GameOverButton, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = style.button_color_pressed.into();
                match button {
                    GameOverButton::Restart => {
                        global_state.set(GlobalState::InGame);
                    }
                    GameOverButton::MainMenu => {
                        global_state.set(GlobalState::MainMenu);
                    }
                    GameOverButton::Settings => {}
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
