use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{utils::remove_all_with, GlobalState};

use super::{
    castle::CastleWall,
    enemies::spawn::EnemySpawnBuffs,
    weapons::{area::Catapulte, projectile::Archer},
    East, GameState, North, Side, South, West,
};

pub struct UpgradesPlugin;

impl Plugin for UpgradesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ApplyUpgrade>()
            .add_event::<ApplyWeaponUpgradeToSide<North>>()
            .add_event::<ApplyWeaponUpgradeToSide<South>>()
            .add_event::<ApplyWeaponUpgradeToSide<West>>()
            .add_event::<ApplyWeaponUpgradeToSide<East>>()
            .add_event::<ApplyEnemyUpgradeToSide<North>>()
            .add_event::<ApplyEnemyUpgradeToSide<South>>()
            .add_event::<ApplyEnemyUpgradeToSide<West>>()
            .add_event::<ApplyEnemyUpgradeToSide<East>>()
            .add_system(setup.in_schedule(OnEnter(GlobalState::InGame)))
            .add_systems(
                (
                    apply_upgrades,
                    apply_weapon_upgrades_to_side::<North>,
                    apply_weapon_upgrades_to_side::<South>,
                    apply_weapon_upgrades_to_side::<West>,
                    apply_weapon_upgrades_to_side::<East>,
                    apply_enemy_upgrades_to_side::<North>,
                    apply_enemy_upgrades_to_side::<South>,
                    apply_enemy_upgrades_to_side::<West>,
                    apply_enemy_upgrades_to_side::<East>,
                )
                    .chain()
                    .in_set(OnUpdate(GameState::LevelUp)),
            )
            .add_system(remove_all_with::<UpgradesMarker>.in_schedule(OnExit(GlobalState::InGame)));
    }
}

#[derive(Component)]
pub struct UpgradesMarker;

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum ApplyUpgrade {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
}

#[derive(Debug, Clone, Copy)]
pub struct ApplyWeaponUpgradeToSide<S: Side> {
    upgrade: WeaponUpgrade,
    _phantom: PhantomData<S>,
}

impl<S: Side> ApplyWeaponUpgradeToSide<S> {
    pub fn new(upgrade: WeaponUpgrade) -> Self {
        Self {
            upgrade,
            _phantom: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ApplyEnemyUpgradeToSide<S: Side> {
    upgrade: EnemyUpgrade,
    _phantom: PhantomData<S>,
}

impl<S: Side> ApplyEnemyUpgradeToSide<S> {
    pub fn new(upgrade: EnemyUpgrade) -> Self {
        Self {
            upgrade,
            _phantom: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WeaponUpgrade {
    AttackSpeed(f32),
    Damage(f32),
}

#[derive(Debug, Clone, Copy)]
pub enum EnemyUpgrade {
    Health(f32),
    MovementSpeed(f32),
}

#[derive(Debug, Clone, Copy)]
pub enum WeaponUpgradeWithSide {
    North(WeaponUpgrade),
    South(WeaponUpgrade),
    West(WeaponUpgrade),
    East(WeaponUpgrade),
}

#[derive(Debug, Clone, Copy)]
pub enum EnemyUpgradeWithSide {
    North(EnemyUpgrade),
    South(EnemyUpgrade),
    West(EnemyUpgrade),
    East(EnemyUpgrade),
}

#[derive(Debug, Clone, Copy)]
pub struct Upgrade {
    weapon_upgrade: WeaponUpgradeWithSide,
    enemy_upgrade: EnemyUpgradeWithSide,
}

/// Here upgrades should be pretty formatted
impl std::fmt::Display for Upgrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Weapon upgrade: ...\n")?;
        f.write_str("Enemy upgrade: ...")?;
        Ok(())
    }
}

#[derive(Component)]
pub struct Upgrades {
    pub upgrades: [Upgrade; 4],
}

fn setup(mut commands: Commands) {
    // generate new upgrades ahead of time
    let upgrades = genereate_upgrades();
    commands.spawn((upgrades, UpgradesMarker));
}

fn genereate_upgrades() -> Upgrades {
    Upgrades {
        upgrades: [
            Upgrade {
                weapon_upgrade: WeaponUpgradeWithSide::North(WeaponUpgrade::AttackSpeed(1.1)),
                enemy_upgrade: EnemyUpgradeWithSide::West(EnemyUpgrade::Health(1.1)),
            },
            Upgrade {
                weapon_upgrade: WeaponUpgradeWithSide::North(WeaponUpgrade::AttackSpeed(1.1)),
                enemy_upgrade: EnemyUpgradeWithSide::West(EnemyUpgrade::Health(1.1)),
            },
            Upgrade {
                weapon_upgrade: WeaponUpgradeWithSide::North(WeaponUpgrade::AttackSpeed(1.1)),
                enemy_upgrade: EnemyUpgradeWithSide::West(EnemyUpgrade::Health(1.1)),
            },
            Upgrade {
                weapon_upgrade: WeaponUpgradeWithSide::North(WeaponUpgrade::AttackSpeed(1.1)),
                enemy_upgrade: EnemyUpgradeWithSide::West(EnemyUpgrade::Health(1.1)),
            },
        ],
    }
}

fn apply_upgrades(
    mut upgrades: Query<&mut Upgrades>,
    mut game_state: ResMut<NextState<GameState>>,
    mut apply_upgrade_events: EventReader<ApplyUpgrade>,
    mut weapon_upgrade_north_event: EventWriter<ApplyWeaponUpgradeToSide<North>>,
    mut weapon_upgrade_south_event: EventWriter<ApplyWeaponUpgradeToSide<South>>,
    mut weapon_upgrade_west_event: EventWriter<ApplyWeaponUpgradeToSide<West>>,
    mut weapon_upgrade_east_event: EventWriter<ApplyWeaponUpgradeToSide<East>>,
    mut enemy_upgrade_north_event: EventWriter<ApplyEnemyUpgradeToSide<North>>,
    mut enemy_upgrade_south_event: EventWriter<ApplyEnemyUpgradeToSide<South>>,
    mut enemy_upgrade_west_event: EventWriter<ApplyEnemyUpgradeToSide<West>>,
    mut enemy_upgrade_east_event: EventWriter<ApplyEnemyUpgradeToSide<East>>,
    mut back_to_in_game: Local<bool>,
) {
    if *back_to_in_game {
        *back_to_in_game = false;
        game_state.set(GameState::InGame);
    }

    let mut upgrades = upgrades.single_mut();
    for event in apply_upgrade_events.iter() {
        let upgrade_to_apply = &upgrades.upgrades[*event as usize];
        // apply
        match upgrade_to_apply.weapon_upgrade {
            WeaponUpgradeWithSide::North(upgrade) => {
                weapon_upgrade_north_event.send(ApplyWeaponUpgradeToSide::new(upgrade))
            }
            WeaponUpgradeWithSide::South(upgrade) => {
                weapon_upgrade_south_event.send(ApplyWeaponUpgradeToSide::new(upgrade))
            }
            WeaponUpgradeWithSide::West(upgrade) => {
                weapon_upgrade_west_event.send(ApplyWeaponUpgradeToSide::new(upgrade))
            }
            WeaponUpgradeWithSide::East(upgrade) => {
                weapon_upgrade_east_event.send(ApplyWeaponUpgradeToSide::new(upgrade))
            }
        }

        match upgrade_to_apply.enemy_upgrade {
            EnemyUpgradeWithSide::North(upgrade) => {
                enemy_upgrade_north_event.send(ApplyEnemyUpgradeToSide::new(upgrade))
            }
            EnemyUpgradeWithSide::South(upgrade) => {
                enemy_upgrade_south_event.send(ApplyEnemyUpgradeToSide::new(upgrade))
            }
            EnemyUpgradeWithSide::West(upgrade) => {
                enemy_upgrade_west_event.send(ApplyEnemyUpgradeToSide::new(upgrade))
            }
            EnemyUpgradeWithSide::East(upgrade) => {
                enemy_upgrade_east_event.send(ApplyEnemyUpgradeToSide::new(upgrade))
            }
        }
        // generate new upgrades ahead of time
        *upgrades = genereate_upgrades();
        *back_to_in_game = true;
    }
}

fn apply_weapon_upgrades_to_side<S: Side>(
    mut _wall: Query<&mut CastleWall<S>>,
    mut _archer: Query<&mut Archer<S>>,
    mut _catapulte: Query<&mut Catapulte<S>>,
    mut weapon_upgrade_events: EventReader<ApplyWeaponUpgradeToSide<S>>,
) {
    for event in weapon_upgrade_events.iter() {
        println!("weapon upgrade event: {event:?}");
    }
}

fn apply_enemy_upgrades_to_side<S: Side>(
    mut _enemy_spawn_buffs: Query<&mut EnemySpawnBuffs<S>>,
    mut enemy_upgrade_events: EventReader<ApplyEnemyUpgradeToSide<S>>,
) {
    for event in enemy_upgrade_events.iter() {
        println!("enemy upgrade event: {event:?}");
    }
}
