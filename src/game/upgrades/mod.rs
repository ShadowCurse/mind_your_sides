use std::marker::PhantomData;

use bevy::prelude::*;

use super::{East, GameState, North, Side, South, West};

pub mod apply;

pub struct UpgradesPlugin;

impl Plugin for UpgradesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(apply::ApplyUpgradesPlugin);
    }
}

macro_rules! random_upgrade {
    ($name:ident, $variant:ident, $type:tt, $min:expr, $max:expr) => {
        pub fn $name(rng: &mut impl rand::Rng) -> Self {
            const MIN: $type = $min;
            const MAX: $type = $max;
            let value = rng.gen_range(MIN..=MAX);
            Self::$variant(value)
        }
    };
}

#[derive(Debug, Clone, Copy)]
pub enum GlobalWallUpgrade {
    AdditionalMaxHp(i32),
    Heal(i32),
}

impl GlobalWallUpgrade {
    random_upgrade!(additional_max_hp, AdditionalMaxHp, i32, 10, 30);
    random_upgrade!(heal, Heal, i32, 20, 70);
}

impl std::fmt::Display for GlobalWallUpgrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AdditionalMaxHp(value) => f.write_fmt(format_args!("max hx: +{value}"))?,
            Self::Heal(value) => f.write_fmt(format_args!("heal: {value}"))?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WallUpgrade {
    AdditionalMaxHp(i32),
    Heal(i32),
}

impl WallUpgrade {
    random_upgrade!(additional_max_hp, AdditionalMaxHp, i32, 20, 50);
    random_upgrade!(heal, Heal, i32, 40, 90);
}

impl std::fmt::Display for WallUpgrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AdditionalMaxHp(value) => f.write_fmt(format_args!("max hx: +{value}"))?,
            Self::Heal(value) => f.write_fmt(format_args!("heal: {value}"))?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GlobalWeaponUpgrade {
    Damage(f32),
    DamageFlat(i32),
    CritDamage(f32),
    CritChance(f32),
}

impl GlobalWeaponUpgrade {
    random_upgrade!(damage, Damage, f32, 2.0, 15.0);
    random_upgrade!(damage_flat, DamageFlat, i32, 20, 50);
    random_upgrade!(crit_dmamge, CritDamage, f32, 5.0, 20.0);
    random_upgrade!(crit_chance, CritChance, f32, 1.0, 10.0);
}

impl std::fmt::Display for GlobalWeaponUpgrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Damage(value) => f.write_fmt(format_args!("damage: +{value:.1}%"))?,
            Self::DamageFlat(value) => f.write_fmt(format_args!("damage: +{value}"))?,
            Self::CritDamage(value) => f.write_fmt(format_args!("crit damage: +{value:.1}%"))?,
            Self::CritChance(value) => f.write_fmt(format_args!("crit chance: +{value:.1}%"))?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WeaponUpgrade {
    CrossbowDamage(f32),
    CrossbowDamageFlat(i32),
    CrossbowCritDamage(f32),
    CrossbowCritChance(f32),
    CrossbowRange(f32),
    CrossbowAttackSpeed(f32),
    CrossbowArrowSpeed(f32),

    MolotovDamage(f32),
    MolotovDamageFlat(i32),
    MolotovCritDamage(f32),
    MolotovCritChance(f32),
    MolotovAreaSize(f32),
    MolotovAttackSpeed(f32),
    MolotovAreaAttackSpeed(f32),
    MolotovAreaLifespan(f32),
}

impl WeaponUpgrade {
    random_upgrade!(crossbow_damage, CrossbowDamage, f32, 1.0, 10.0);
    random_upgrade!(crossbow_damage_flat, CrossbowDamageFlat, i32, 5, 70);
    random_upgrade!(crossbow_crit_damage, CrossbowCritDamage, f32, 1.0, 15.0);
    random_upgrade!(crossbow_crit_chance, CrossbowCritChance, f32, 1.0, 10.0);
    random_upgrade!(crossbow_range, CrossbowRange, f32, 10.0, 100.0);
    random_upgrade!(crossbow_attack_speed, CrossbowAttackSpeed, f32, 1.0, 5.0);
    random_upgrade!(crossbow_arrow_speed, CrossbowArrowSpeed, f32, 5.0, 20.0);

    random_upgrade!(molotov_damage, MolotovDamage, f32, 1.0, 8.0);
    random_upgrade!(molotov_damage_flat, MolotovDamageFlat, i32, 3, 40);
    random_upgrade!(molotov_crit_damage, MolotovCritDamage, f32, 1.0, 5.0);
    random_upgrade!(molotov_crit_chance, MolotovCritChance, f32, 10.0, 10.0);
    random_upgrade!(molotov_area_size, MolotovAreaSize, f32, 5.0, 20.0);
    random_upgrade!(molotov_attack_speed, MolotovAttackSpeed, f32, 1.0, 5.0);
    random_upgrade!(
        molotov_area_attack_speed,
        MolotovAreaAttackSpeed,
        f32,
        1.0,
        5.0
    );
    random_upgrade!(molotov_area_lifespan, MolotovAreaLifespan, f32, 1.0, 3.0);
}

#[rustfmt::skip]
impl std::fmt::Display for WeaponUpgrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CrossbowDamage(value) => f.write_fmt(format_args!("crossbow damage: +{value:.1}%"))?,
            Self::CrossbowDamageFlat(value) => f.write_fmt(format_args!("crossbow damage: +{value}"))?,
            Self::CrossbowCritDamage(value) => f.write_fmt(format_args!("crossbow crit damage: +{value:.1}%"))?,
            Self::CrossbowCritChance(value) => f.write_fmt(format_args!("crossbow crit chance: +{value:.1}%"))?,
            Self::CrossbowRange(value) => f.write_fmt(format_args!("crossbow range: +{value:.1}%"))?,
            Self::CrossbowAttackSpeed(value) => f.write_fmt(format_args!("crossbow attack speed: +{value:.1}%"))?,
            Self::CrossbowArrowSpeed(value) => f.write_fmt(format_args!("crossbow arrow speed: +{value:.1}%"))?,
            Self::MolotovDamage(value) => f.write_fmt(format_args!("molotov damage: +{value:.1}%"))?,
            Self::MolotovDamageFlat(value) => f.write_fmt(format_args!("molotov damage: +{value}"))?,
            Self::MolotovCritDamage(value) => f.write_fmt(format_args!("molotov crit damage: +{value:.1}%"))?,
            Self::MolotovCritChance(value) => f.write_fmt(format_args!("molotov crit chance: +{value:.1}%"))?,
            Self::MolotovAreaSize(value) => f.write_fmt(format_args!("molotov area size: +{value:.1}%"))?,
            Self::MolotovAttackSpeed(value) => f.write_fmt(format_args!("molotov attack speed: +{value:.1}%"))?,
            Self::MolotovAreaAttackSpeed(value) => f.write_fmt(format_args!("molotov area attack speed: +{value:.1}%"))?,
            Self::MolotovAreaLifespan(value) => f.write_fmt(format_args!("molotov area lifespan: +{value:.1}%"))?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GlobalEnemyUpgrade {
    Health(f32),
    Speed(f32),
    Exp(f32),
    Damage(f32),
    AttackSpeed(f32),
}

impl GlobalEnemyUpgrade {
    random_upgrade!(health, Health, f32, 1.0, 10.0);
    random_upgrade!(speed, Speed, f32, 0.1, 1.0);
    random_upgrade!(exp, Exp, f32, 1.0, 8.0);
    random_upgrade!(damage, Damage, f32, 2.0, 10.0);
    random_upgrade!(attack_speed, AttackSpeed, f32, 1.0, 5.0);
}

impl std::fmt::Display for GlobalEnemyUpgrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Health(value) => f.write_fmt(format_args!("health: +{value:.1}%"))?,
            Self::Speed(value) => f.write_fmt(format_args!("movement speed: +{value:.1}%"))?,
            Self::Exp(value) => f.write_fmt(format_args!("exp drop: -{value:.1}%"))?,
            Self::Damage(value) => f.write_fmt(format_args!("damage: +{value:.1}%"))?,
            Self::AttackSpeed(value) => f.write_fmt(format_args!("attack speed: +{value:.1}%"))?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EnemyUpgrade {
    Health(f32),
    Speed(f32),
    Exp(f32),
    Damage(f32),
    AttackSpeed(f32),
}

impl std::fmt::Display for EnemyUpgrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Health(value) => f.write_fmt(format_args!("health: +{value:.1}%"))?,
            Self::Speed(value) => f.write_fmt(format_args!("movement speed: +{value:.1}%"))?,
            Self::Exp(value) => f.write_fmt(format_args!("exp drop: -{value:.1}%"))?,
            Self::Damage(value) => f.write_fmt(format_args!("damage: +{value:.1}%"))?,
            Self::AttackSpeed(value) => f.write_fmt(format_args!("attack speed: +{value:.1}%"))?,
        }
        Ok(())
    }
}

impl EnemyUpgrade {
    random_upgrade!(health, Health, f32, 1.0, 20.0);
    random_upgrade!(speed, Speed, f32, 0.1, 5.0);
    random_upgrade!(exp, Exp, f32, 1.0, 15.0);
    random_upgrade!(damage, Damage, f32, 2.0, 25.0);
    random_upgrade!(attack_speed, AttackSpeed, f32, 1.0, 10.0);
}

#[derive(Debug, Clone, Copy)]
pub enum UpgradeSide<U> {
    North(U),
    South(U),
    West(U),
    East(U),
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Upgrade {
    pub global_wall_upgrade: Option<GlobalWallUpgrade>,
    pub wall_upgrade: Option<UpgradeSide<WallUpgrade>>,
    pub global_weapon_upgrade: Option<GlobalWeaponUpgrade>,
    pub weapon_upgrade: Option<UpgradeSide<WeaponUpgrade>>,
    pub global_enemy_upgrade: Option<GlobalEnemyUpgrade>,
    pub enemy_upgrade: Option<UpgradeSide<EnemyUpgrade>>,
}

pub struct GlobalBuffs {
    pub wall_upgrade: Option<GlobalWallUpgrade>,
    pub weapon_upgrade: Option<GlobalWeaponUpgrade>,
}

impl std::fmt::Display for GlobalBuffs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(upgrade) = self.wall_upgrade {
            f.write_fmt(format_args!("{upgrade}\n"))?;
        }
        if let Some(upgrade) = self.weapon_upgrade {
            f.write_fmt(format_args!("{upgrade}\n"))?;
        }
        Ok(())
    }
}

pub struct GlobalDebuffs {
    pub enemy_upgrade: Option<GlobalEnemyUpgrade>,
}

impl std::fmt::Display for GlobalDebuffs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(upgrade) = self.enemy_upgrade {
            f.write_fmt(format_args!("{upgrade}\n"))?;
        }
        Ok(())
    }
}

pub struct SideBuffs<S: Side> {
    pub wall_upgrade: Option<WallUpgrade>,
    pub weapon_upgrade: Option<WeaponUpgrade>,
    _phantom: PhantomData<S>,
}

impl<S: Side> SideBuffs<S> {
    pub fn new(wall_upgrade: Option<WallUpgrade>, weapon_upgrade: Option<WeaponUpgrade>) -> Self {
        Self {
            wall_upgrade,
            weapon_upgrade,
            _phantom: PhantomData,
        }
    }
}

impl<S: Side> std::fmt::Display for SideBuffs<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(upgrade) = self.wall_upgrade {
            f.write_fmt(format_args!("{upgrade}\n"))?;
        }
        if let Some(upgrade) = self.weapon_upgrade {
            f.write_fmt(format_args!("{upgrade}\n"))?;
        }
        Ok(())
    }
}

pub struct SideDebuffs<S: Side> {
    pub enemy_upgrade: Option<EnemyUpgrade>,
    _phantom: PhantomData<S>,
}

impl<S: Side> SideDebuffs<S> {
    pub fn new(enemy_upgrade: Option<EnemyUpgrade>) -> Self {
        Self {
            enemy_upgrade,
            _phantom: PhantomData,
        }
    }
}

impl<S: Side> std::fmt::Display for SideDebuffs<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(upgrade) = self.enemy_upgrade {
            f.write_fmt(format_args!("{upgrade}\n"))?;
        }
        Ok(())
    }
}

macro_rules! get_side {
    ($self:ident, $field:ident, $side:ident) => {
        $self
            .$field
            .map(|upgrade| match upgrade {
                UpgradeSide::$side(upgrade) => Some(upgrade),
                _ => None,
            })
            .flatten()
    };
}

impl Upgrade {
    pub fn has_global_upgrades(&self) -> bool {
        self.global_wall_upgrade.is_some()
            || self.global_weapon_upgrade.is_some()
            || self.global_enemy_upgrade.is_some()
    }

    pub fn global_upgrades(&self) -> (GlobalBuffs, GlobalDebuffs) {
        (
            GlobalBuffs {
                wall_upgrade: self.global_wall_upgrade,
                weapon_upgrade: self.global_weapon_upgrade,
            },
            GlobalDebuffs {
                enemy_upgrade: self.global_enemy_upgrade,
            },
        )
    }

    pub fn has_north_upgrades(&self) -> bool {
        let wall_upgrade = get_side!(self, wall_upgrade, North).is_some();
        let weapon_upgrade = get_side!(self, weapon_upgrade, North).is_some();
        let enemy_upgrade = get_side!(self, enemy_upgrade, North).is_some();
        wall_upgrade || weapon_upgrade || enemy_upgrade
    }

    pub fn north_upgrades(&self) -> (SideBuffs<North>, SideDebuffs<North>) {
        let wall_upgrade = get_side!(self, wall_upgrade, North);
        let weapon_upgrade = get_side!(self, weapon_upgrade, North);
        let enemy_upgrade = get_side!(self, enemy_upgrade, North);
        (
            SideBuffs::new(wall_upgrade, weapon_upgrade),
            SideDebuffs::new(enemy_upgrade),
        )
    }

    pub fn has_south_upgrades(&self) -> bool {
        let wall_upgrade = get_side!(self, wall_upgrade, South).is_some();
        let weapon_upgrade = get_side!(self, weapon_upgrade, South).is_some();
        let enemy_upgrade = get_side!(self, enemy_upgrade, South).is_some();
        wall_upgrade || weapon_upgrade || enemy_upgrade
    }

    pub fn south_upgrades(&self) -> (SideBuffs<North>, SideDebuffs<North>) {
        let wall_upgrade = get_side!(self, wall_upgrade, South);
        let weapon_upgrade = get_side!(self, weapon_upgrade, South);
        let enemy_upgrade = get_side!(self, enemy_upgrade, South);
        (
            SideBuffs::new(wall_upgrade, weapon_upgrade),
            SideDebuffs::new(enemy_upgrade),
        )
    }

    pub fn has_west_upgrades(&self) -> bool {
        let wall_upgrade = get_side!(self, wall_upgrade, West).is_some();
        let weapon_upgrade = get_side!(self, weapon_upgrade, West).is_some();
        let enemy_upgrade = get_side!(self, enemy_upgrade, West).is_some();
        wall_upgrade || weapon_upgrade || enemy_upgrade
    }

    pub fn west_upgrades(&self) -> (SideBuffs<West>, SideDebuffs<West>) {
        let wall_upgrade = get_side!(self, wall_upgrade, West);
        let weapon_upgrade = get_side!(self, weapon_upgrade, West);
        let enemy_upgrade = get_side!(self, enemy_upgrade, West);
        (
            SideBuffs::new(wall_upgrade, weapon_upgrade),
            SideDebuffs::new(enemy_upgrade),
        )
    }

    pub fn has_east_upgrades(&self) -> bool {
        let wall_upgrade = get_side!(self, wall_upgrade, East).is_some();
        let weapon_upgrade = get_side!(self, weapon_upgrade, East).is_some();
        let enemy_upgrade = get_side!(self, enemy_upgrade, East).is_some();
        wall_upgrade || weapon_upgrade || enemy_upgrade
    }

    pub fn east_upgrades(&self) -> (SideBuffs<East>, SideDebuffs<East>) {
        let wall_upgrade = get_side!(self, wall_upgrade, East);
        let weapon_upgrade = get_side!(self, weapon_upgrade, East);
        let enemy_upgrade = get_side!(self, enemy_upgrade, East);
        (
            SideBuffs::new(wall_upgrade, weapon_upgrade),
            SideDebuffs::new(enemy_upgrade),
        )
    }
}

/// Here upgrades should be pretty formatted
impl std::fmt::Display for Upgrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(upgrade) = self.global_wall_upgrade {
            f.write_fmt(format_args!("Global:\n\twalls {upgrade}\n"))?;
        }
        if let Some(upgrade) = self.wall_upgrade {
            match upgrade {
                UpgradeSide::North(upgrade) => {
                    f.write_fmt(format_args!("North wall:\n\t{upgrade}\n"))?;
                }
                UpgradeSide::South(upgrade) => {
                    f.write_fmt(format_args!("South wall:\n\t{upgrade}\n"))?;
                }
                UpgradeSide::West(upgrade) => {
                    f.write_fmt(format_args!("West wall:\n\t{upgrade}\n"))?;
                }
                UpgradeSide::East(upgrade) => {
                    f.write_fmt(format_args!("East wall:\n\t{upgrade}\n"))?;
                }
            }
        }

        if let Some(upgrade) = self.global_weapon_upgrade {
            f.write_fmt(format_args!("Global weapons:\n\t{upgrade}\n"))?;
        }
        if let Some(upgrade) = self.weapon_upgrade {
            match upgrade {
                UpgradeSide::North(upgrade) => {
                    f.write_fmt(format_args!("North:\n\t{upgrade}\n"))?;
                }
                UpgradeSide::South(upgrade) => {
                    f.write_fmt(format_args!("South:\n\t{upgrade}\n"))?;
                }
                UpgradeSide::West(upgrade) => {
                    f.write_fmt(format_args!("West:\n\t{upgrade}\n"))?;
                }
                UpgradeSide::East(upgrade) => {
                    f.write_fmt(format_args!("East:\n\t{upgrade}\n"))?;
                }
            }
        }

        if let Some(upgrade) = self.global_enemy_upgrade {
            f.write_fmt(format_args!("Global enemies:\n\t{upgrade}\n"))?;
        }
        if let Some(upgrade) = self.enemy_upgrade {
            match upgrade {
                UpgradeSide::North(upgrade) => {
                    f.write_fmt(format_args!("North enemies:\n\t{upgrade}\n"))?;
                }
                UpgradeSide::South(upgrade) => {
                    f.write_fmt(format_args!("South enemies:\n\t{upgrade}\n"))?;
                }
                UpgradeSide::West(upgrade) => {
                    f.write_fmt(format_args!("West enemies:\n\t{upgrade}\n"))?;
                }
                UpgradeSide::East(upgrade) => {
                    f.write_fmt(format_args!("East enemies:\n\t{upgrade}\n"))?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default, Resource)]
pub struct Upgrades {
    pub upgrades: [Upgrade; 4],
}

pub fn genereate_upgrades() -> Upgrades {
    let mut rng = rand::thread_rng();
    Upgrades {
        upgrades: [
            genereate_upgrade(&mut rng),
            genereate_upgrade(&mut rng),
            genereate_upgrade(&mut rng),
            genereate_upgrade(&mut rng),
        ],
    }
}

pub fn genereate_upgrade(mut rng: &mut impl rand::Rng) -> Upgrade {
    // wall
    let (global_wall_upgrade, wall_upgrade, must_have_weapon) = if rng.gen_ratio(2, 10) {
        if rng.gen_ratio(3, 10) {
            let upgrade = match rng.gen_range(0..1) {
                0 => GlobalWallUpgrade::additional_max_hp(&mut rng),
                1 => GlobalWallUpgrade::heal(&mut rng),
                _ => unreachable!(),
            };
            (Some(upgrade), None, false)
        } else {
            // one side wall
            let upgrade = match rng.gen_range(0..2) {
                0 => WallUpgrade::additional_max_hp(&mut rng),
                1 => WallUpgrade::heal(&mut rng),
                _ => unreachable!(),
            };

            let upgrade = match rng.gen_range(0..4) {
                0 => UpgradeSide::North(upgrade),
                1 => UpgradeSide::South(upgrade),
                2 => UpgradeSide::West(upgrade),
                3 => UpgradeSide::East(upgrade),
                _ => unreachable!(),
            };
            (None, Some(upgrade), false)
        }
    } else {
        (None, None, true)
    };

    // weapon
    let (global_weapon_upgrade, weapon_upgrade) = if rng.gen_ratio(9, 10) || must_have_weapon {
        if rng.gen_ratio(3, 10) {
            let upgrade = match rng.gen_range(0..4) {
                0 => GlobalWeaponUpgrade::damage(&mut rng),
                1 => GlobalWeaponUpgrade::damage_flat(&mut rng),
                2 => GlobalWeaponUpgrade::crit_dmamge(&mut rng),
                3 => GlobalWeaponUpgrade::crit_chance(&mut rng),
                _ => unreachable!(),
            };
            (Some(upgrade), None)
        } else {
            // one side wall
            let upgrade = match rng.gen_range(0..15) {
                0 => WeaponUpgrade::crossbow_damage(&mut rng),
                1 => WeaponUpgrade::crossbow_damage_flat(&mut rng),
                2 => WeaponUpgrade::crossbow_crit_damage(&mut rng),
                3 => WeaponUpgrade::crossbow_crit_chance(&mut rng),
                4 => WeaponUpgrade::crossbow_range(&mut rng),
                5 => WeaponUpgrade::crossbow_attack_speed(&mut rng),
                6 => WeaponUpgrade::crossbow_arrow_speed(&mut rng),

                7 => WeaponUpgrade::molotov_damage(&mut rng),
                8 => WeaponUpgrade::molotov_damage_flat(&mut rng),
                9 => WeaponUpgrade::molotov_crit_damage(&mut rng),
                10 => WeaponUpgrade::molotov_crit_chance(&mut rng),
                11 => WeaponUpgrade::molotov_area_size(&mut rng),
                12 => WeaponUpgrade::molotov_attack_speed(&mut rng),
                13 => WeaponUpgrade::molotov_area_attack_speed(&mut rng),
                14 => WeaponUpgrade::molotov_area_lifespan(&mut rng),
                _ => unreachable!(),
            };

            let upgrade = match rng.gen_range(0..4) {
                0 => UpgradeSide::North(upgrade),
                1 => UpgradeSide::South(upgrade),
                2 => UpgradeSide::West(upgrade),
                3 => UpgradeSide::East(upgrade),
                _ => unreachable!(),
            };
            (None, Some(upgrade))
        }
    } else {
        (None, None)
    };

    // enemy
    let (global_enemy_upgrade, enemy_upgrade) = if rng.gen_ratio(99, 100) {
        if rng.gen_ratio(3, 10) {
            let upgrade = match rng.gen_range(0..5) {
                0 => GlobalEnemyUpgrade::health(&mut rng),
                1 => GlobalEnemyUpgrade::speed(&mut rng),
                2 => GlobalEnemyUpgrade::exp(&mut rng),
                3 => GlobalEnemyUpgrade::damage(&mut rng),
                4 => GlobalEnemyUpgrade::attack_speed(&mut rng),
                _ => unreachable!(),
            };
            (Some(upgrade), None)
        } else {
            // one side wall
            let upgrade = match rng.gen_range(0..5) {
                0 => EnemyUpgrade::health(&mut rng),
                1 => EnemyUpgrade::speed(&mut rng),
                2 => EnemyUpgrade::exp(&mut rng),
                3 => EnemyUpgrade::damage(&mut rng),
                4 => EnemyUpgrade::attack_speed(&mut rng),
                _ => unreachable!(),
            };

            let upgrade = match rng.gen_range(0..4) {
                0 => UpgradeSide::North(upgrade),
                1 => UpgradeSide::South(upgrade),
                2 => UpgradeSide::West(upgrade),
                3 => UpgradeSide::East(upgrade),
                _ => unreachable!(),
            };
            (None, Some(upgrade))
        }
    } else {
        (None, None)
    };

    Upgrade {
        global_wall_upgrade,
        wall_upgrade,
        global_weapon_upgrade,
        weapon_upgrade,
        global_enemy_upgrade,
        enemy_upgrade,
    }
}
