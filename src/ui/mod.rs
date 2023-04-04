use bevy::prelude::*;

use crate::{GameAssets, GlobalState};

mod in_game;
mod main_menu;

const BASE_FONT_SIZE: f32 = 21.0;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_ui_config.in_schedule(OnEnter(GlobalState::Initialization)))
            .add_plugin(in_game::UiInGamePlugin)
            .add_plugin(main_menu::UiMainMenuPlugin);
    }
}

#[derive(Debug, Clone, Resource)]
pub struct UiConfig {
    pub button_style: Style,
    pub button_color_normal: Color,
    pub button_color_hover: Color,
    pub button_color_pressed: Color,
    pub menu_style: Style,
    pub menu_color: Color,
    pub text_style: TextStyle,
    pub title_style: Style,
    pub title_text_style: TextStyle,
}

fn setup_ui_config(game_assets: Res<GameAssets>, mut commands: Commands) {
    commands.insert_resource(UiConfig {
        button_style: Style {
            size: Size::new(Val::Px(140.0), Val::Px(60.0)),
            margin: UiRect::all(Val::Percent(1.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        button_color_normal: Color::DARK_GRAY,
        button_color_hover: Color::ORANGE,
        button_color_pressed: Color::GREEN,
        menu_style: Style {
            flex_direction: FlexDirection::Column,
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        menu_color: Color::NONE,
        text_style: TextStyle {
            font: game_assets.font.clone(),
            font_size: BASE_FONT_SIZE * 2.0,
            color: Color::BLACK,
        },
        title_style: Style {
            margin: UiRect::bottom(Val::Percent(30.0)),
            ..default()
        },
        title_text_style: TextStyle {
            font: game_assets.font.clone(),
            font_size: BASE_FONT_SIZE * 4.0,
            color: Color::BLACK,
        },
    });
}

fn spawn_button<B>(child_builder: &mut ChildBuilder, style: &UiConfig, button: B)
where
    B: Component + std::fmt::Debug + Copy,
{
    child_builder
        .spawn((
            ButtonBundle {
                style: style.button_style.clone(),
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
