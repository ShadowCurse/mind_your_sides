use crate::game::upgrades::apply::ApplyUpgradeEvent;
use crate::game::upgrades::{Upgrade, Upgrades};
use crate::ui::in_game::UiInGameState;
use crate::ui::UiConfig;
use crate::utils::remove_all_with;

use bevy::prelude::*;

use super::hud::HUDMarker;

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

fn setup(
    ui_config: Res<UiConfig>,
    upgrades: Res<Upgrades>,
    hud: Query<Entity, With<HUDMarker>>,
    mut commands: Commands,
) {
    let level_up = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            LevelUpMarker,
        ))
        .with_children(|builder| {
            builder.spawn(
                TextBundle::from_section("Level up", ui_config.title_text_style.clone())
                    .with_style(Style {
                        margin: UiRect::all(Val::Percent(4.0)),
                        ..default()
                    }),
            );
            builder
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                margin: UiRect::all(Val::Percent(1.0)),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|builder| {
                            spawn_upgrade_button(
                                builder,
                                &ui_config,
                                UpgradeButton::First,
                                &upgrades.upgrades[0],
                            );
                            spawn_upgrade_button(
                                builder,
                                &ui_config,
                                UpgradeButton::Second,
                                &upgrades.upgrades[1],
                            );
                        });
                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                margin: UiRect::all(Val::Percent(1.0)),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|builder| {
                            spawn_upgrade_button(
                                builder,
                                &ui_config,
                                UpgradeButton::Third,
                                &upgrades.upgrades[2],
                            );
                            spawn_upgrade_button(
                                builder,
                                &ui_config,
                                UpgradeButton::Fourth,
                                &upgrades.upgrades[3],
                            );
                        });
                });
        })
        .id();

    let hud = hud.single();
    commands.entity(hud).insert_children(1, &[level_up]);
}

fn button_system(
    config: Res<UiConfig>,
    mut apply_upgrade_event: EventWriter<ApplyUpgradeEvent>,
    mut interaction_query: Query<
        (&UpgradeButton, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = config.button_color_pressed.into();
                let event = match button {
                    UpgradeButton::First => ApplyUpgradeEvent::First,
                    UpgradeButton::Second => ApplyUpgradeEvent::Second,
                    UpgradeButton::Third => ApplyUpgradeEvent::Third,
                    UpgradeButton::Fourth => ApplyUpgradeEvent::Fourth,
                };
                apply_upgrade_event.send(event);
            }
            Interaction::Hovered => {
                *color = config.button_color_hover.into();
            }
            Interaction::None => {
                *color = config.button_color_normal.into();
            }
        }
    }
}

fn spawn_upgrade_button<B>(
    builder: &mut ChildBuilder,
    style: &UiConfig,
    button: B,
    upgrade: &Upgrade,
) where
    B: Component + std::fmt::Debug + Copy,
{
    builder
        .spawn((
            ButtonBundle {
                style: style.upgrade_button_style.clone(),
                background_color: style.button_color_normal.into(),
                ..default()
            },
            button,
        ))
        .with_children(|builder| {
            // Global
            if upgrade.has_global_upgrades() {
                let (buffs, debuffs) = upgrade.global_upgrades();

                builder.spawn(TextBundle {
                    text: Text::from_section("Global:", style.text_style.clone()),
                    ..default()
                });
                builder.spawn(TextBundle {
                    text: Text::from_section(format!("{buffs}"), style.buff_text_style.clone()),
                    ..default()
                });
                builder.spawn(TextBundle {
                    text: Text::from_section(format!("{debuffs}"), style.debuff_text_style.clone()),
                    ..default()
                });
            }
            // North
            if upgrade.has_north_upgrades() {
                let (buffs, debuffs) = upgrade.north_upgrades();

                builder.spawn(TextBundle {
                    text: Text::from_section("North:", style.text_style.clone()),
                    ..default()
                });
                builder.spawn(TextBundle {
                    text: Text::from_section(format!("{buffs}"), style.buff_text_style.clone()),
                    ..default()
                });
                builder.spawn(TextBundle {
                    text: Text::from_section(format!("{debuffs}"), style.debuff_text_style.clone()),
                    ..default()
                });
            }
            // South
            if upgrade.has_south_upgrades() {
                let (buffs, debuffs) = upgrade.south_upgrades();

                builder.spawn(TextBundle {
                    text: Text::from_section("South:", style.text_style.clone()),
                    ..default()
                });
                builder.spawn(TextBundle {
                    text: Text::from_section(format!("{buffs}"), style.buff_text_style.clone()),
                    ..default()
                });
                builder.spawn(TextBundle {
                    text: Text::from_section(format!("{debuffs}"), style.debuff_text_style.clone()),
                    ..default()
                });
            }
            // West
            if upgrade.has_west_upgrades() {
                let (buffs, debuffs) = upgrade.west_upgrades();

                builder.spawn(TextBundle {
                    text: Text::from_section("West:", style.text_style.clone()),
                    ..default()
                });
                builder.spawn(TextBundle {
                    text: Text::from_section(format!("{buffs}"), style.buff_text_style.clone()),
                    ..default()
                });
                builder.spawn(TextBundle {
                    text: Text::from_section(format!("{debuffs}"), style.debuff_text_style.clone()),
                    ..default()
                });
            }
            // East
            if upgrade.has_east_upgrades() {
                let (buffs, debuffs) = upgrade.east_upgrades();

                builder.spawn(TextBundle {
                    text: Text::from_section("East:", style.text_style.clone()),
                    ..default()
                });
                builder.spawn(TextBundle {
                    text: Text::from_section(format!("{buffs}"), style.buff_text_style.clone()),
                    ..default()
                });
                builder.spawn(TextBundle {
                    text: Text::from_section(format!("{debuffs}"), style.debuff_text_style.clone()),
                    ..default()
                });
            }
        });
}
