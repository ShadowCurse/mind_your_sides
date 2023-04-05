use crate::game::upgrades::apply::ApplyUpgradeEvent;
use crate::game::upgrades::Upgrades;
use crate::ui::in_game::UiInGameState;
use crate::ui::UiConfig;
use crate::utils::remove_all_with;

use bevy::prelude::*;

pub struct LevelUpPlugin;

impl Plugin for LevelUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(UiInGameState::LevelUp)))
            .add_system(button_system.in_set(OnUpdate(UiInGameState::LevelUp)))
            .add_system(
                remove_all_with::<LevelUpMarker>.in_schedule(OnExit(UiInGameState::LevelUp)),
            );
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, Component)]
enum UpgradeButton {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Debug, Clone, Copy, Component)]
struct LevelUpMarker;

fn setup(ui_config: Res<UiConfig>, upgrades: Res<Upgrades>, mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: ui_config.menu_style.clone(),
                background_color: ui_config.menu_color.into(),
                ..default()
            },
            LevelUpMarker,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Buffs",
                ui_config.title_text_style.clone(),
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
                    background_color: ui_config.menu_color.into(),
                    ..default()
                })
                .with_children(|builder| {
                    spawn_upgrade_button(
                        builder,
                        &ui_config,
                        UpgradeButton::First,
                        format!("{}", upgrades.upgrades[0]),
                    );
                    spawn_upgrade_button(
                        builder,
                        &ui_config,
                        UpgradeButton::Second,
                        format!("{}", upgrades.upgrades[1]),
                    );
                    spawn_upgrade_button(
                        builder,
                        &ui_config,
                        UpgradeButton::Third,
                        format!("{}", upgrades.upgrades[2]),
                    );
                    spawn_upgrade_button(
                        builder,
                        &ui_config,
                        UpgradeButton::Fourth,
                        format!("{}", upgrades.upgrades[3]),
                    );
                });
        });
}

fn button_system(
    style: Res<UiConfig>,
    mut apply_upgrade_event: EventWriter<ApplyUpgradeEvent>,
    mut interaction_query: Query<
        (&UpgradeButton, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = style.button_color_pressed.into();
                let event = match button {
                    UpgradeButton::First => ApplyUpgradeEvent::First,
                    UpgradeButton::Second => ApplyUpgradeEvent::Second,
                    UpgradeButton::Third => ApplyUpgradeEvent::Third,
                    UpgradeButton::Fourth => ApplyUpgradeEvent::Fourth,
                };
                apply_upgrade_event.send(event);
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

fn spawn_upgrade_button<B>(
    child_builder: &mut ChildBuilder,
    style: &UiConfig,
    button: B,
    text: String,
) where
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
                text: Text::from_section(text, style.text_style.clone()),
                ..default()
            });
        });
}
