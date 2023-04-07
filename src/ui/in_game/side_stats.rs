use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{
    game::{
        enemies::{spawn::EnemyBuffs, GlobalEnemyBuffs},
        weapons::{
            crossbow::{Crossbow, CrossbowBuffs},
            molotov::{Molotov, MolotovBuffs},
            GlobalWeaponBuffs,
        },
        East, GameState, North, Side, South, West,
    },
    ui::{spawn_button, UiConfig},
    utils::remove_all_with,
};

use super::{hud::HUDMarker, UiInGameState};

#[derive(Debug, Default, Clone, Copy)]
pub struct StatsPlugin<S: Side> {
    _phantom: PhantomData<S>,
}

impl Plugin for StatsPlugin<North> {
    fn build(&self, app: &mut App) {
        app.add_system(setup::<North>.in_schedule(OnEnter(UiInGameState::StatsNorth)))
            .add_system(button_system::<North>.in_set(OnUpdate(UiInGameState::StatsNorth)))
            .add_system(
                remove_all_with::<StatsMarker<North>>
                    .in_schedule(OnExit(UiInGameState::StatsNorth)),
            );
    }
}

impl Plugin for StatsPlugin<South> {
    fn build(&self, app: &mut App) {
        app.add_system(setup::<South>.in_schedule(OnEnter(UiInGameState::StatsSouth)))
            .add_system(button_system::<South>.in_set(OnUpdate(UiInGameState::StatsSouth)))
            .add_system(
                remove_all_with::<StatsMarker<South>>
                    .in_schedule(OnExit(UiInGameState::StatsSouth)),
            );
    }
}

impl Plugin for StatsPlugin<West> {
    fn build(&self, app: &mut App) {
        app.add_system(setup::<West>.in_schedule(OnEnter(UiInGameState::StatsWest)))
            .add_system(button_system::<West>.in_set(OnUpdate(UiInGameState::StatsWest)))
            .add_system(
                remove_all_with::<StatsMarker<West>>.in_schedule(OnExit(UiInGameState::StatsWest)),
            );
    }
}

impl Plugin for StatsPlugin<East> {
    fn build(&self, app: &mut App) {
        app.add_system(setup::<East>.in_schedule(OnEnter(UiInGameState::StatsEast)))
            .add_system(button_system::<East>.in_set(OnUpdate(UiInGameState::StatsEast)))
            .add_system(
                remove_all_with::<StatsMarker<East>>.in_schedule(OnExit(UiInGameState::StatsEast)),
            );
    }
}

#[derive(Debug, Default, Clone, Copy, Component)]
struct StatsMarker<S: Side> {
    _phantom: PhantomData<S>,
}

#[derive(Debug, Clone, Copy, Component)]
enum StatsButton {
    Back,
}

fn setup<S: Side>(
    config: Res<UiConfig>,
    hud: Query<Entity, With<HUDMarker>>,
    global_weapons_buffs: Res<GlobalWeaponBuffs>,
    corssbow_buffs: Res<CrossbowBuffs<S>>,
    molotov_buffs: Res<MolotovBuffs<S>>,
    global_enemy_buffs: Res<GlobalEnemyBuffs>,
    enemy_buffs: Res<EnemyBuffs<S>>,
    mut commands: Commands,
) {
    let stats = commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new( Val::Px(900.0),Val::Px(600.0)),
                    padding: UiRect::all(Val::Percent(2.0)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: config.panels_background.into(),
                ..default()
            },
            StatsMarker::<S>::default(),
        ))
        .with_children(|builder| {
            builder.spawn( NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(230.0), Val::Px(500.0)),
                    padding: UiRect::all(Val::Percent(2.0)),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            }).with_children( |builder|{
                builder.spawn( NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(230.0), Val::Px(500.0)),
                        padding: UiRect::all(Val::Percent(2.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                }).with_children(|builder|{
                    builder.spawn(TextBundle {
                        text: Text::from_section("North:", config.text_style.clone()),
                        ..default()
                    });
                });
                builder.spawn( NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(230.0), Val::Px(500.0)),
                        padding: UiRect::all(Val::Percent(2.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                }).with_children(|builder|{
                    builder.spawn(TextBundle {
                        text: Text::from_section("Crossbow:", config.text_style.clone()),
                        ..default()
                    });

                    let buffed_crossbow =
                        Crossbow::default().with_buffs(&corssbow_buffs, &global_weapons_buffs);
                    builder.spawn(TextBundle {
                        text: Text::from_section(
                            format!("{buffed_crossbow}"),
                            config.buff_text_style.clone(),
                        ),
                        ..default()
                    });
                });
                builder.spawn( NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(230.0), Val::Px(500.0)),
                        padding: UiRect::all(Val::Percent(2.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                }).with_children(|builder|{
                    builder.spawn(TextBundle {
                        text: Text::from_section("Molotov:", config.text_style.clone()),
                        ..default()
                    });

                    let buffed_molotov =
                        Molotov::default().with_buffs(&molotov_buffs, &global_weapons_buffs);
                    builder.spawn(TextBundle {
                        text: Text::from_section(
                            format!("{buffed_molotov}"),
                            config.buff_text_style.clone(),
                        ),
                        ..default()
                    });

                });
                builder.spawn( NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(230.0), Val::Px(500.0)),
                        padding: UiRect::all(Val::Percent(2.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                }).with_children(|builder|{
                    builder.spawn(TextBundle {
                        text: Text::from_section("Enemy:", config.text_style.clone()),
                        ..default()
                    });
                    let enemy_buffs = enemy_buffs.with_global_buffs(&global_enemy_buffs);
                    builder.spawn(TextBundle {
                        text: Text::from_section(
                            format!("{enemy_buffs}"),
                            config.debuff_text_style.clone(),
                        ),
                        ..default()
                    });
            }); });
            spawn_button(builder, &config, StatsButton::Back);
        })
        .id();

    let hud = hud.single();
    commands.entity(hud).insert_children(1, &[stats]);
}

fn button_system<S: Side>(
    style: Res<UiConfig>,
    mut game_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StatsButton>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                game_state.set(GameState::InGame);
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
