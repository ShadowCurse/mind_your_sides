use bevy::prelude::*;

use crate::{GameAssets, GlobalState};

pub mod in_game;
pub mod main_menu;

pub struct UiPlugin;

const UI_SCALE: f32 = 1.0;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_ui_config.in_schedule(OnEnter(GlobalState::Initialization)))
            .add_plugin(in_game::UiInGamePlugin)
            .add_plugin(main_menu::UiMainMenuPlugin);
    }
}

#[derive(Debug, Clone, Resource)]
pub struct UiConfig {
    pub clear_background: Color,
    pub panels_background: Color,
    pub button_color_normal: Color,
    pub button_color_hover: Color,
    pub button_color_pressed: Color,
    pub button_style: Style,
    pub upgrade_button_style: Style,
    pub menu_style: Style,
    pub stats_style: Style,
    pub title_style: Style,
    pub text_style: TextStyle,
    pub buff_text_style: TextStyle,
    pub debuff_text_style: TextStyle,
    pub title_text_style: TextStyle,
}

fn setup_ui_config(game_assets: Res<GameAssets>, mut commands: Commands) {
    let _light_grey = Color::rgb_u8(192, 203, 220);
    let _medium_grey = Color::rgb_u8(139, 155, 180);
    let light_blue = Color::rgb_u8(90, 105, 136);
    let medium_blue = Color::rgb_u8(58, 68, 102);
    let dark_blue = Color::rgb_u8(38, 43, 68);
    let darker_blue = Color::rgb_u8(30, 37, 60);

    commands.insert_resource(UiConfig {
        clear_background: Color::NONE,
        panels_background: dark_blue,
        button_color_normal: darker_blue,
        button_color_hover: medium_blue,
        button_color_pressed: light_blue,
        button_style: Style {
            size: Size::new(Val::Px(180.0), Val::Px(80.0)),
            margin: UiRect::all(Val::Percent(1.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        upgrade_button_style: Style {
            size: Size::new(Val::Px(350.0), Val::Px(250.0)),
            margin: UiRect::all(Val::Percent(1.0)),
            padding: UiRect::all(Val::Percent(3.0)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        menu_style: Style {
            size: Size::new(Val::Px(500.0), Val::Px(400.0)),
            margin: UiRect::all(Val::Auto),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        stats_style: Style {
            size: Size::new(Val::Percent(8.9 / 16.0 * 100.0), Val::Px(400.0)),
            padding: UiRect::all(Val::Percent(2.0)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Center,
            ..default()
        },
        title_style: Style {
            margin: UiRect::bottom(Val::Percent(8.0)),
            ..default()
        },
        text_style: TextStyle {
            font: game_assets.font.clone(),
            font_size: 35.0,
            color: Color::WHITE,
        },
        buff_text_style: TextStyle {
            font: game_assets.font.clone(),
            font_size: 24.0,
            color: Color::rgb_u8(62, 137, 72),
        },
        debuff_text_style: TextStyle {
            font: game_assets.font.clone(),
            font_size: 24.0,
            color: Color::rgb_u8(228, 59, 68),
        },
        title_text_style: TextStyle {
            font: game_assets.font.clone(),
            font_size: 40.0,
            color: Color::WHITE,
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
