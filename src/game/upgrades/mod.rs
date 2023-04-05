use bevy::prelude::*;
use rand::Rng;

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
    random_upgrade!(additional_max_hp, AdditionalMaxHp, i32, 10, 30);
    random_upgrade!(heal, Heal, i32, 20, 70);
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
    random_upgrade!(damage, Damage, f32, 20.0, 70.0);
    random_upgrade!(damage_flat, DamageFlat, i32, 20, 70);
    random_upgrade!(crit_dmamge, CritDamage, f32, 20.0, 70.0);
    random_upgrade!(crit_chance, CritChance, f32, 20.0, 70.0);
}

impl std::fmt::Display for GlobalWeaponUpgrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Damage(value) => f.write_fmt(format_args!("damage: +{value:.0}%"))?,
            Self::DamageFlat(value) => f.write_fmt(format_args!("damage: +{value}"))?,
            Self::CritDamage(value) => f.write_fmt(format_args!("crit damage: +{value:.0}%"))?,
            Self::CritChance(value) => f.write_fmt(format_args!("crit chance: +{value:.0}%"))?,
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
    random_upgrade!(crossbow_damage, CrossbowDamage, f32, 20.0, 70.0);
    random_upgrade!(crossbow_damage_flat, CrossbowDamageFlat, i32, 20, 70);
    random_upgrade!(crossbow_crit_damage, CrossbowCritDamage, f32, 20.0, 70.0);
    random_upgrade!(crossbow_crit_chance, CrossbowCritChance, f32, 20.0, 70.0);
    random_upgrade!(crossbow_range, CrossbowRange, f32, 20.0, 70.0);
    random_upgrade!(crossbow_attack_speed, CrossbowAttackSpeed, f32, 20.0, 70.0);
    random_upgrade!(crossbow_arrow_speed, CrossbowArrowSpeed, f32, 20.0, 70.0);

    random_upgrade!(molotov_damage, MolotovDamage, f32, 20.0, 70.0);
    random_upgrade!(molotov_damage_flat, MolotovDamageFlat, i32, 20, 70);
    random_upgrade!(molotov_crit_damage, MolotovCritDamage, f32, 20.0, 70.0);
    random_upgrade!(molotov_crit_chance, MolotovCritChance, f32, 20.0, 70.0);
    random_upgrade!(molotov_area_size, MolotovAreaSize, f32, 20.0, 70.0);
    random_upgrade!(molotov_attack_speed, MolotovAttackSpeed, f32, 20.0, 70.0);
    random_upgrade!(
        molotov_area_attack_speed,
        MolotovAreaAttackSpeed,
        f32,
        20.0,
        70.0
    );
    random_upgrade!(molotov_area_lifespan, MolotovAreaLifespan, f32, 20.0, 70.0);
}

#[rustfmt::skip]
impl std::fmt::Display for WeaponUpgrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CrossbowDamage(value) => f.write_fmt(format_args!("crossbow damage: +{value:.0}%"))?,
            Self::CrossbowDamageFlat(value) => f.write_fmt(format_args!("crossbow damage: +{value}"))?,
            Self::CrossbowCritDamage(value) => f.write_fmt(format_args!("crossbow crit damage: +{value:.0}%"))?,
            Self::CrossbowCritChance(value) => f.write_fmt(format_args!("crossbow crit chance: +{value:.0}%"))?,
            Self::CrossbowRange(value) => f.write_fmt(format_args!("crossbow range: +{value:.0}%"))?,
            Self::CrossbowAttackSpeed(value) => f.write_fmt(format_args!("crossbow attack speed: +{value:.0}%"))?,
            Self::CrossbowArrowSpeed(value) => f.write_fmt(format_args!("crossbow arrow spped: +{value:.0}%"))?,
            Self::MolotovDamage(value) => f.write_fmt(format_args!("molotov damage: +{value:.0}%"))?,
            Self::MolotovDamageFlat(value) => f.write_fmt(format_args!("molotov damage: +{value}"))?,
            Self::MolotovCritDamage(value) => f.write_fmt(format_args!("molotov crit damage: +{value:.0}%"))?,
            Self::MolotovCritChance(value) => f.write_fmt(format_args!("molotov crit chance: +{value:.0}%"))?,
            Self::MolotovAreaSize(value) => f.write_fmt(format_args!("molotov area size: +{value:.0}%"))?,
            Self::MolotovAttackSpeed(value) => f.write_fmt(format_args!("molotov attack speed: +{value:.0}%"))?,
            Self::MolotovAreaAttackSpeed(value) => f.write_fmt(format_args!("molotov area attack speed: +{value:.0}%"))?,
            Self::MolotovAreaLifespan(value) => f.write_fmt(format_args!("molotov area lifespan: +{value:.0}%"))?,
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
    random_upgrade!(health, Health, f32, 20.0, 70.0);
    random_upgrade!(speed, Speed, f32, 20.0, 70.0);
    random_upgrade!(exp, Exp, f32, 20.0, 70.0);
    random_upgrade!(damage, Damage, f32, 20.0, 70.0);
    random_upgrade!(attack_speed, AttackSpeed, f32, 20.0, 70.0);
}

impl std::fmt::Display for GlobalEnemyUpgrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Health(value) => f.write_fmt(format_args!("health: +{value:.0}%"))?,
            Self::Speed(value) => f.write_fmt(format_args!("movement speed: +{value:.0}%"))?,
            Self::Exp(value) => f.write_fmt(format_args!("exp drop: +{value:.0}%"))?,
            Self::Damage(value) => f.write_fmt(format_args!("damage: +{value:.0}%"))?,
            Self::AttackSpeed(value) => f.write_fmt(format_args!("attack speed: +{value:.0}%"))?,
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
            Self::Health(value) => f.write_fmt(format_args!("health: +{value:.0}%"))?,
            Self::Speed(value) => f.write_fmt(format_args!("movement speed: +{value:.0}%"))?,
            Self::Exp(value) => f.write_fmt(format_args!("exp drop: +{value:.0}%"))?,
            Self::Damage(value) => f.write_fmt(format_args!("damage: +{value:.0}%"))?,
            Self::AttackSpeed(value) => f.write_fmt(format_args!("attack speed: +{value:.0}%"))?,
        }
        Ok(())
    }
}

impl EnemyUpgrade {
    random_upgrade!(health, Health, f32, 20.0, 70.0);
    random_upgrade!(speed, Speed, f32, 20.0, 70.0);
    random_upgrade!(exp, Exp, f32, 20.0, 70.0);
    random_upgrade!(damage, Damage, f32, 20.0, 70.0);
    random_upgrade!(attack_speed, AttackSpeed, f32, 20.0, 70.0);
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
    let (global_wall_upgrade, wall_upgrade) = if rng.gen_ratio(8, 10) {
        if rng.gen_ratio(3, 10) {
            let upgrade = match rng.gen_range(0..1) {
                0 => GlobalWallUpgrade::additional_max_hp(&mut rng),
                1 => GlobalWallUpgrade::heal(&mut rng),
                _ => unreachable!(),
            };
            (Some(upgrade), None)
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
            (None, Some(upgrade))
        }
    } else {
        (None, None)
    };

    // weapon
    let (global_weapon_upgrade, weapon_upgrade) = if rng.gen_ratio(2, 10) {
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
    let (global_enemy_upgrade, enemy_upgrade) = if rng.gen_ratio(9, 10) {
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
