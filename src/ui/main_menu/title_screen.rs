use bevy::{app::AppExit, prelude::*};

use crate::{utils::remove_all_with, GlobalState};

use super::{spawn_button, UiConfig, UiMainMenuState};

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(UiMainMenuState::TitleScreen)))
            .add_system(button_system.in_set(OnUpdate(UiMainMenuState::TitleScreen)))
            .add_system(
                remove_all_with::<TitleScreenMarker>
                    .in_schedule(OnExit(UiMainMenuState::TitleScreen)),
            );
    }
}

#[derive(Debug, Clone, Copy, Component)]
struct TitleScreenMarker;

#[derive(Debug, Clone, Copy, Component)]
enum TitleScreenButton {
    Start,
    Settings,
    Exit,
}

fn setup(mut commands: Commands, config: Res<UiConfig>) {
    commands
        .spawn(NodeBundle {
            style: config.menu_style.clone(),
            background_color: config.menu_color.into(),
            ..default()
        })
        .insert(TitleScreenMarker)
        .with_children(|builder| {
            spawn_button(
                builder,
                &config,
                TitleScreenButton::Start,
                TitleScreenMarker,
            );
            spawn_button(
                builder,
                &config,
                TitleScreenButton::Settings,
                TitleScreenMarker,
            );
            spawn_button(builder, &config, TitleScreenButton::Exit, TitleScreenMarker);
        });
}

fn button_system(
    style: Res<UiConfig>,
    mut main_menu_state: ResMut<NextState<UiMainMenuState>>,
    mut global_state: ResMut<NextState<GlobalState>>,
    mut interaction_query: Query<
        (&TitleScreenButton, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut exit: EventWriter<AppExit>,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = style.button_color_pressed.into();
                match button {
                    TitleScreenButton::Start => {
                        global_state.set(GlobalState::InGame);
                    }
                    TitleScreenButton::Settings => {
                        main_menu_state.set(UiMainMenuState::Settings);
                    }
                    TitleScreenButton::Exit => exit.send(AppExit),
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
