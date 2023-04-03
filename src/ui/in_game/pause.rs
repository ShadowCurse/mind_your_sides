use bevy::prelude::*;

use crate::{
    game::GameState,
    ui::{spawn_button, UiConfig},
    utils::remove_all_with,
    GlobalState,
};

use super::UiInGameState;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(UiInGameState::Pause)))
            .add_system(button_system.in_set(OnUpdate(UiInGameState::Pause)))
            .add_system(remove_all_with::<PauseMarker>.in_schedule(OnExit(UiInGameState::Pause)));
    }
}

#[derive(Debug, Clone, Copy, Component)]
struct PauseMarker;

#[derive(Debug, Clone, Copy, Component)]
enum PauseButton {
    MainMenu,
    Settings,
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
            PauseMarker,
        ))
        .with_children(|builder| {
            spawn_button(builder, &config, PauseButton::MainMenu);
            spawn_button(builder, &config, PauseButton::Settings);
            spawn_button(builder, &config, PauseButton::Back);
        });
}

fn button_system(
    style: Res<UiConfig>,
    mut global_state: ResMut<NextState<GlobalState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&PauseButton, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = style.button_color_pressed.into();
                match button {
                    PauseButton::MainMenu => {
                        global_state.set(GlobalState::MainMenu);
                    }
                    PauseButton::Settings => {}
                    PauseButton::Back => {
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
