use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{
    game::{
        castle::CastleWall,
        enemies::{spawn::EnemyBuffs, GlobalEnemyBuffs},
        weapons::{crossbow::CrossbowBuffs, molotov::MolotovBuffs, GlobalWeaponBuffs},
    },
    GlobalState,
};

use super::{East, GameState, North, Side, South, West, *};

pub struct ApplyUpgradesPlugin;

impl Plugin for ApplyUpgradesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ApplyUpgradeEvent>()
            .add_event::<FinishUpgradeEvent>()
            .add_event::<GlobalWallUpgradeEvent>()
            .add_event::<WallUpgradeEvent<North>>()
            .add_event::<WallUpgradeEvent<South>>()
            .add_event::<WallUpgradeEvent<West>>()
            .add_event::<WallUpgradeEvent<East>>()
            .add_event::<GlobalWeaponUpgradeEvent>()
            .add_event::<WeaponUpgradeEvent<North>>()
            .add_event::<WeaponUpgradeEvent<South>>()
            .add_event::<WeaponUpgradeEvent<West>>()
            .add_event::<WeaponUpgradeEvent<East>>()
            .add_event::<GlobalEnemyUpgradeEvent>()
            .add_event::<EnemyUpgradeEvent<North>>()
            .add_event::<EnemyUpgradeEvent<South>>()
            .add_event::<EnemyUpgradeEvent<West>>()
            .add_event::<EnemyUpgradeEvent<East>>()
            .configure_set(UpgradeSet::Dispatch.before(UpgradeSet::Apply))
            .configure_set(UpgradeSet::Apply.before(UpgradeSet::Finish))
            .add_system(setup.in_schedule(OnEnter(GlobalState::InGame)))
            .add_systems(
                (
                    dispatch_wall_upgrades,
                    dispatch_weapon_upgrades,
                    dispatch_enemy_upgrades,
                )
                    .in_set(UpgradeSet::Dispatch)
                    .in_set(OnUpdate(GameState::LevelUp)),
            )
            .add_systems(
                (
                    apply_global_wall_upgrades,
                    apply_wall_upgrades_to_side::<North>,
                    apply_wall_upgrades_to_side::<South>,
                    apply_wall_upgrades_to_side::<West>,
                    apply_wall_upgrades_to_side::<East>,
                    apply_global_weapon_upgrades,
                    apply_weapon_upgrades_to_side::<North>,
                    apply_weapon_upgrades_to_side::<South>,
                    apply_weapon_upgrades_to_side::<West>,
                    apply_weapon_upgrades_to_side::<East>,
                    apply_global_enemy_upgrades,
                    apply_enemy_upgrades_to_side::<North>,
                    apply_enemy_upgrades_to_side::<South>,
                    apply_enemy_upgrades_to_side::<West>,
                    apply_enemy_upgrades_to_side::<East>,
                )
                    .in_set(UpgradeSet::Apply)
                    .in_set(OnUpdate(GameState::LevelUp)),
            )
            .add_system(
                finish_upgrade
                    .in_set(UpgradeSet::Finish)
                    .in_set(OnUpdate(GameState::LevelUp)),
            );
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
enum UpgradeSet {
    Dispatch,
    Apply,
    Finish,
}

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum ApplyUpgradeEvent {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
}

pub struct FinishUpgradeEvent;

#[derive(Debug, Clone, Copy)]
pub struct GlobalWallUpgradeEvent {
    upgrade: GlobalWallUpgrade,
}

impl GlobalWallUpgradeEvent {
    pub fn new(upgrade: GlobalWallUpgrade) -> Self {
        Self { upgrade }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WallUpgradeEvent<S: Side> {
    upgrade: WallUpgrade,
    _phantom: PhantomData<S>,
}

impl<S: Side> WallUpgradeEvent<S> {
    pub fn new(upgrade: WallUpgrade) -> Self {
        Self {
            upgrade,
            _phantom: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GlobalWeaponUpgradeEvent {
    upgrade: GlobalWeaponUpgrade,
}

impl GlobalWeaponUpgradeEvent {
    pub fn new(upgrade: GlobalWeaponUpgrade) -> Self {
        Self { upgrade }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WeaponUpgradeEvent<S: Side> {
    upgrade: WeaponUpgrade,
    _phantom: PhantomData<S>,
}

impl<S: Side> WeaponUpgradeEvent<S> {
    pub fn new(upgrade: WeaponUpgrade) -> Self {
        Self {
            upgrade,
            _phantom: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GlobalEnemyUpgradeEvent {
    upgrade: GlobalEnemyUpgrade,
}

impl GlobalEnemyUpgradeEvent {
    pub fn new(upgrade: GlobalEnemyUpgrade) -> Self {
        Self { upgrade }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EnemyUpgradeEvent<S: Side> {
    upgrade: EnemyUpgrade,
    _phantom: PhantomData<S>,
}

impl<S: Side> EnemyUpgradeEvent<S> {
    pub fn new(upgrade: EnemyUpgrade) -> Self {
        Self {
            upgrade,
            _phantom: PhantomData,
        }
    }
}

fn setup(mut commands: Commands) {
    // generate new upgrades ahead of time
    commands.insert_resource(genereate_upgrades());
}

fn finish_upgrade(
    finish_events: EventReader<FinishUpgradeEvent>,
    mut upgrades: ResMut<Upgrades>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if !finish_events.is_empty() {
        *upgrades = genereate_upgrades();
        game_state.set(GameState::InGame);
    }
}

fn dispatch_wall_upgrades(
    upgrades: Res<Upgrades>,
    mut apply_upgrade_events: EventReader<ApplyUpgradeEvent>,
    mut global_wall_upgrade_event: EventWriter<GlobalWallUpgradeEvent>,
    mut wall_upgrade_north_event: EventWriter<WallUpgradeEvent<North>>,
    mut wall_upgrade_south_event: EventWriter<WallUpgradeEvent<South>>,
    mut wall_upgrade_west_event: EventWriter<WallUpgradeEvent<West>>,
    mut wall_upgrade_east_event: EventWriter<WallUpgradeEvent<East>>,
) {
    for event in apply_upgrade_events.iter() {
        let upgrade_to_apply = &upgrades.upgrades[*event as usize];

        if let Some(upgrade) = upgrade_to_apply.global_wall_upgrade {
            global_wall_upgrade_event.send(GlobalWallUpgradeEvent::new(upgrade))
        }
        if let Some(upgrade) = upgrade_to_apply.wall_upgrade {
            match upgrade {
                UpgradeSide::North(upgrade) => {
                    wall_upgrade_north_event.send(WallUpgradeEvent::new(upgrade))
                }
                UpgradeSide::South(upgrade) => {
                    wall_upgrade_south_event.send(WallUpgradeEvent::new(upgrade))
                }
                UpgradeSide::West(upgrade) => {
                    wall_upgrade_west_event.send(WallUpgradeEvent::new(upgrade))
                }
                UpgradeSide::East(upgrade) => {
                    wall_upgrade_east_event.send(WallUpgradeEvent::new(upgrade))
                }
            }
        }
    }
}

fn dispatch_weapon_upgrades(
    upgrades: Res<Upgrades>,
    mut apply_upgrade_events: EventReader<ApplyUpgradeEvent>,
    mut global_weapon_upgrade_event: EventWriter<GlobalWeaponUpgradeEvent>,
    mut weapon_upgrade_north_event: EventWriter<WeaponUpgradeEvent<North>>,
    mut weapon_upgrade_south_event: EventWriter<WeaponUpgradeEvent<South>>,
    mut weapon_upgrade_west_event: EventWriter<WeaponUpgradeEvent<West>>,
    mut weapon_upgrade_east_event: EventWriter<WeaponUpgradeEvent<East>>,
) {
    for event in apply_upgrade_events.iter() {
        let upgrade_to_apply = &upgrades.upgrades[*event as usize];

        if let Some(upgrade) = upgrade_to_apply.global_weapon_upgrade {
            global_weapon_upgrade_event.send(GlobalWeaponUpgradeEvent::new(upgrade))
        }
        if let Some(upgrade) = upgrade_to_apply.weapon_upgrade {
            match upgrade {
                UpgradeSide::North(upgrade) => {
                    weapon_upgrade_north_event.send(WeaponUpgradeEvent::new(upgrade))
                }
                UpgradeSide::South(upgrade) => {
                    weapon_upgrade_south_event.send(WeaponUpgradeEvent::new(upgrade))
                }
                UpgradeSide::West(upgrade) => {
                    weapon_upgrade_west_event.send(WeaponUpgradeEvent::new(upgrade))
                }
                UpgradeSide::East(upgrade) => {
                    weapon_upgrade_east_event.send(WeaponUpgradeEvent::new(upgrade))
                }
            }
        }
    }
}

fn dispatch_enemy_upgrades(
    upgrades: Res<Upgrades>,
    mut apply_upgrade_events: EventReader<ApplyUpgradeEvent>,
    mut global_enemy_upgrade_event: EventWriter<GlobalEnemyUpgradeEvent>,
    mut enemy_upgrade_north_event: EventWriter<EnemyUpgradeEvent<North>>,
    mut enemy_upgrade_south_event: EventWriter<EnemyUpgradeEvent<South>>,
    mut enemy_upgrade_west_event: EventWriter<EnemyUpgradeEvent<West>>,
    mut enemy_upgrade_east_event: EventWriter<EnemyUpgradeEvent<East>>,
) {
    for event in apply_upgrade_events.iter() {
        let upgrade_to_apply = &upgrades.upgrades[*event as usize];

        if let Some(upgrade) = upgrade_to_apply.global_enemy_upgrade {
            global_enemy_upgrade_event.send(GlobalEnemyUpgradeEvent::new(upgrade))
        }
        if let Some(upgrade) = upgrade_to_apply.enemy_upgrade {
            match upgrade {
                UpgradeSide::North(upgrade) => {
                    enemy_upgrade_north_event.send(EnemyUpgradeEvent::new(upgrade))
                }
                UpgradeSide::South(upgrade) => {
                    enemy_upgrade_south_event.send(EnemyUpgradeEvent::new(upgrade))
                }
                UpgradeSide::West(upgrade) => {
                    enemy_upgrade_west_event.send(EnemyUpgradeEvent::new(upgrade))
                }
                UpgradeSide::East(upgrade) => {
                    enemy_upgrade_east_event.send(EnemyUpgradeEvent::new(upgrade))
                }
            }
        }
    }
}

fn apply_global_wall_upgrades(
    mut north_wall: Query<&mut CastleWall<North>>,
    mut south_wall: Query<&mut CastleWall<South>>,
    mut west_wall: Query<&mut CastleWall<West>>,
    mut east_wall: Query<&mut CastleWall<East>>,
    mut global_wall_upgrade_events: EventReader<GlobalWallUpgradeEvent>,
    mut finish_event: EventWriter<FinishUpgradeEvent>,
) {
    let mut north_wall = north_wall.single_mut();
    let mut south_wall = south_wall.single_mut();
    let mut west_wall = west_wall.single_mut();
    let mut east_wall = east_wall.single_mut();
    for event in global_wall_upgrade_events.iter() {
        match event.upgrade {
            GlobalWallUpgrade::AdditionalMaxHp(value) => {
                north_wall.add_max_hp(value);
                south_wall.add_max_hp(value);
                west_wall.add_max_hp(value);
                east_wall.add_max_hp(value);
            }
            GlobalWallUpgrade::Heal(value) => {
                north_wall.heal(value);
                south_wall.heal(value);
                west_wall.heal(value);
                east_wall.heal(value);
            }
        }
        finish_event.send(FinishUpgradeEvent);
    }
}

fn apply_wall_upgrades_to_side<S: Side>(
    mut wall: Query<&mut CastleWall<S>>,
    mut wall_upgrade_events: EventReader<WallUpgradeEvent<S>>,
    mut finish_event: EventWriter<FinishUpgradeEvent>,
) {
    let mut wall = wall.single_mut();
    for event in wall_upgrade_events.iter() {
        match event.upgrade {
            WallUpgrade::AdditionalMaxHp(value) => wall.add_max_hp(value),
            WallUpgrade::Heal(value) => wall.heal(value),
        }
        finish_event.send(FinishUpgradeEvent);
    }
}

fn apply_global_weapon_upgrades(
    mut global_weapons_buffs: ResMut<GlobalWeaponBuffs>,
    mut global_weapon_upgrade_events: EventReader<GlobalWeaponUpgradeEvent>,
    mut finish_event: EventWriter<FinishUpgradeEvent>,
) {
    for event in global_weapon_upgrade_events.iter() {
        match event.upgrade {
            GlobalWeaponUpgrade::Damage(value) => global_weapons_buffs.damage += value / 100.0,
            GlobalWeaponUpgrade::DamageFlat(value) => global_weapons_buffs.damage_flat += value,
            GlobalWeaponUpgrade::CritDamage(value) => {
                global_weapons_buffs.crit_damage += value / 100.0
            }
            GlobalWeaponUpgrade::CritChance(value) => {
                global_weapons_buffs.crit_chance += value / 100.0
            }
        }
        finish_event.send(FinishUpgradeEvent);
    }
}

fn apply_weapon_upgrades_to_side<S: Side>(
    mut crossbow_buffs: ResMut<CrossbowBuffs<S>>,
    mut molotov_buffs: ResMut<MolotovBuffs<S>>,
    mut weapon_upgrade_events: EventReader<WeaponUpgradeEvent<S>>,
    mut finish_event: EventWriter<FinishUpgradeEvent>,
) {
    for event in weapon_upgrade_events.iter() {
        match event.upgrade {
            WeaponUpgrade::CrossbowDamage(value) => crossbow_buffs.damage += value / 100.0,
            WeaponUpgrade::CrossbowDamageFlat(value) => crossbow_buffs.damage_flat += value,
            WeaponUpgrade::CrossbowCritDamage(value) => crossbow_buffs.crit_damage += value / 100.0,
            WeaponUpgrade::CrossbowCritChance(value) => crossbow_buffs.crit_chance += value / 100.0,
            WeaponUpgrade::CrossbowRange(value) => crossbow_buffs.range += value / 100.0,
            WeaponUpgrade::CrossbowAttackSpeed(value) => {
                crossbow_buffs.attack_speed += value / 100.0
            }
            WeaponUpgrade::CrossbowArrowSpeed(value) => crossbow_buffs.arrow_speed += value / 100.0,
            WeaponUpgrade::MolotovDamage(value) => molotov_buffs.damage += value / 100.0,
            WeaponUpgrade::MolotovDamageFlat(value) => molotov_buffs.damage_flat += value,
            WeaponUpgrade::MolotovCritDamage(value) => molotov_buffs.crit_damage += value / 100.0,
            WeaponUpgrade::MolotovCritChance(value) => molotov_buffs.crit_chance += value / 100.0,
            WeaponUpgrade::MolotovAreaSize(value) => molotov_buffs.area_size += value / 100.0,
            WeaponUpgrade::MolotovAttackSpeed(value) => molotov_buffs.attack_speed += value / 100.0,
            WeaponUpgrade::MolotovAreaAttackSpeed(value) => {
                molotov_buffs.area_attack_speed += value
            }
            WeaponUpgrade::MolotovAreaLifespan(value) => {
                molotov_buffs.area_lifespan += value / 100.0
            }
        }
        finish_event.send(FinishUpgradeEvent);
    }
}

fn apply_global_enemy_upgrades(
    mut global_enemy_buffs: ResMut<GlobalEnemyBuffs>,
    mut global_enemy_upgrade_events: EventReader<GlobalEnemyUpgradeEvent>,
    mut finish_event: EventWriter<FinishUpgradeEvent>,
) {
    for event in global_enemy_upgrade_events.iter() {
        match event.upgrade {
            GlobalEnemyUpgrade::Health(value) => global_enemy_buffs.health += value / 100.0,
            GlobalEnemyUpgrade::Speed(value) => global_enemy_buffs.speed += value / 100.0,
            GlobalEnemyUpgrade::Exp(value) => global_enemy_buffs.exp -= value / 100.0,
            GlobalEnemyUpgrade::Damage(value) => global_enemy_buffs.damage += value / 100.0,
            GlobalEnemyUpgrade::AttackSpeed(value) => {
                global_enemy_buffs.attack_speed += value / 100.0
            }
        }
        finish_event.send(FinishUpgradeEvent);
    }
}

fn apply_enemy_upgrades_to_side<S: Side>(
    mut enemy_spawn_buffs: ResMut<EnemyBuffs<S>>,
    mut enemy_upgrade_events: EventReader<EnemyUpgradeEvent<S>>,
    mut finish_event: EventWriter<FinishUpgradeEvent>,
) {
    for event in enemy_upgrade_events.iter() {
        match event.upgrade {
            EnemyUpgrade::Health(value) => enemy_spawn_buffs.health += value / 100.0,
            EnemyUpgrade::Speed(value) => enemy_spawn_buffs.speed += value / 100.0,
            EnemyUpgrade::Exp(value) => enemy_spawn_buffs.exp -= value / 100.0,
            EnemyUpgrade::Damage(value) => enemy_spawn_buffs.damage += value / 100.0,
            EnemyUpgrade::AttackSpeed(value) => enemy_spawn_buffs.attack_speed += value / 100.0,
        }
        finish_event.send(FinishUpgradeEvent);
    }
}
