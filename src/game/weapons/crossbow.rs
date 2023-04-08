use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{
    game::{
        damage::projectile::ProjectileBundle, enemies::Enemy, East, GameState, North, Side, South,
        West,
    },
    utils::remove_all_with,
    GameAssets, GameSettings, GlobalState,
};

use super::{GlobalWeaponBuffs, WeaponsAssets};

const DEFAULT_BOLT_SIZE: f32 = 3.0;
const DEFAULT_BOLT_SPEED: f32 = 200.0;

const DEFAULT_CROSSBOW_DAMAGE: i32 = 20;
const DEFAULT_CROSSBOW_CRIT_DAMAGE: f32 = 2.0;
const DEFAULT_CROSSBOW_CRIT_CHANCE: f32 = 0.05;
const DEFAULT_CROSSBOW_RANGE: f32 = 500.0;
const DEFAULT_CROSSBOW_ATTACK_SPEED: f32 = 1.0;

/// Offsets arrow spawn point in the enemy direction
const DEFAULT_BOLT_SPAWN_OFFSET: f32 = 30.0;

pub struct CrossbowPlugin;

impl Plugin for CrossbowPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GlobalState::InGame)))
            .add_systems(
                (
                    crossbow_attack::<North>,
                    crossbow_attack::<South>,
                    crossbow_attack::<West>,
                    crossbow_attack::<East>,
                )
                    .in_set(OnUpdate(GameState::InGame)),
            )
            .add_system(remove_all_with::<CrossbowMarker>.in_schedule(OnExit(GlobalState::InGame)));
    }
}

#[derive(Component)]
pub struct CrossbowMarker;

#[derive(Debug, Default, Resource)]
pub struct CrossbowBuffs<S: Side> {
    pub damage: f32,
    pub damage_flat: i32,
    pub crit_damage: f32,
    pub crit_chance: f32,
    pub range: f32,
    pub attack_speed: f32,
    pub arrow_speed: f32,
    _phantom: PhantomData<S>,
}

#[derive(Component)]
pub struct Crossbow<S: Side> {
    damage: i32,
    range: f32,
    crit_damage: f32,
    crit_chance: f32,
    arrow_speed: f32,
    attack_timer: Timer,
    _phantom: PhantomData<S>,
}

impl<S: Side> std::fmt::Display for Crossbow<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("damage {}\n", self.damage))?;
        f.write_fmt(format_args!("range {:.1}\n", self.range))?;
        f.write_fmt(format_args!(
            "crit damage {:.1}%\n",
            self.crit_damage * 100.0
        ))?;
        f.write_fmt(format_args!(
            "crit chance {:.1}%\n",
            self.crit_chance * 100.0
        ))?;
        f.write_fmt(format_args!("arrow speed {:.1}\n", self.arrow_speed))?;
        f.write_fmt(format_args!(
            "attack speed {:.1}/s\n",
            self.attack_timer.duration().as_secs_f32()
        ))?;
        Ok(())
    }
}

impl<S: Side> Default for Crossbow<S> {
    fn default() -> Self {
        Self {
            damage: DEFAULT_CROSSBOW_DAMAGE,
            range: DEFAULT_CROSSBOW_RANGE,
            crit_damage: DEFAULT_CROSSBOW_CRIT_DAMAGE,
            crit_chance: DEFAULT_CROSSBOW_CRIT_CHANCE,
            arrow_speed: DEFAULT_BOLT_SPEED,
            attack_timer: Timer::from_seconds(DEFAULT_CROSSBOW_ATTACK_SPEED, TimerMode::Repeating),
            _phantom: PhantomData,
        }
    }
}

impl<S: Side> Crossbow<S> {
    pub fn with_buffs(
        self,
        crossbow_buffs: &CrossbowBuffs<S>,
        global_weapons_buffs: &GlobalWeaponBuffs,
    ) -> Self {
        Self {
            damage: ((self.damage + crossbow_buffs.damage_flat + global_weapons_buffs.damage_flat)
                as f32
                * (1.0 + crossbow_buffs.damage + global_weapons_buffs.damage))
                as i32,
            range: self.range * (1.0 + crossbow_buffs.range),
            crit_damage: self.crit_damage
                + crossbow_buffs.crit_damage
                + global_weapons_buffs.crit_damage,
            crit_chance: self.crit_chance
                + crossbow_buffs.crit_chance
                + global_weapons_buffs.crit_chance,
            arrow_speed: self.arrow_speed * (1.0 + crossbow_buffs.arrow_speed),
            attack_timer: Timer::from_seconds(
                DEFAULT_CROSSBOW_ATTACK_SPEED * (1.0 + crossbow_buffs.attack_speed),
                TimerMode::Repeating,
            ),
            _phantom: PhantomData,
        }
    }
}

#[derive(Bundle)]
pub struct CrossbowBundle<S: Side> {
    crossbow: Crossbow<S>,
    marker: CrossbowMarker,
}

impl<S: Side> Default for CrossbowBundle<S> {
    fn default() -> Self {
        Self {
            crossbow: Default::default(),
            marker: CrossbowMarker,
        }
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(CrossbowBuffs::<North>::default());
    commands.insert_resource(CrossbowBuffs::<South>::default());
    commands.insert_resource(CrossbowBuffs::<West>::default());
    commands.insert_resource(CrossbowBuffs::<East>::default());
}

fn crossbow_attack<S: Side>(
    time: Res<Time>,
    audio: Res<Audio>,
    game_assets: Res<GameAssets>,
    game_settings: Res<GameSettings>,
    weapon_assets: Res<WeaponsAssets>,
    crossbow_buffs: Res<CrossbowBuffs<S>>,
    global_weapons_buffs: Res<GlobalWeaponBuffs>,
    enemies: Query<&Transform, With<Enemy<S>>>,
    mut commands: Commands,
    mut crossbows: Query<(&Transform, &mut Crossbow<S>)>,
) {
    for (transform, mut crossbow) in crossbows.iter_mut() {
        if !crossbow.attack_timer.tick(time.delta()).finished() {
            continue;
        }

        crossbow.attack_timer = Timer::from_seconds(
            DEFAULT_CROSSBOW_ATTACK_SPEED * (1.0 + crossbow_buffs.attack_speed),
            TimerMode::Repeating,
        );

        let mut enemy_vec = Vec2::default();
        let mut min_range = crossbow.range;
        for enemy_transform in enemies.iter() {
            let vec = (enemy_transform.translation - transform.translation).truncate();
            let distance = vec.length();
            if distance < min_range {
                min_range = distance;
                enemy_vec = vec;
            }
        }

        // no enemies in range
        let range = crossbow.range * (1.0 + crossbow_buffs.range);
        if range <= min_range {
            continue;
        }

        let direction = enemy_vec.normalize();
        let mut projectile_transform = *transform;
        projectile_transform.translation += (direction * DEFAULT_BOLT_SPAWN_OFFSET).extend(0.0);

        // rotates arrow in the enemy direaction
        // arorw sprite looks to the left == NEG_X
        let arrow_direction = Vec2::NEG_X;
        projectile_transform.rotate_z(-direction.angle_between(arrow_direction));

        let damage =
            ((crossbow.damage + crossbow_buffs.damage_flat + global_weapons_buffs.damage_flat)
                as f32
                * (1.0 + crossbow_buffs.damage + global_weapons_buffs.damage)) as i32;
        let arrow_speed = crossbow.arrow_speed * (1.0 + crossbow_buffs.arrow_speed);
        let crit_chance =
            crossbow.crit_chance + crossbow_buffs.crit_chance + global_weapons_buffs.crit_chance;
        let crit_damage = (crossbow.damage as f32
            * (crossbow.crit_damage
                + crossbow_buffs.crit_damage
                + global_weapons_buffs.crit_damage)) as i32;

        commands.spawn(ProjectileBundle::<S>::new(
            weapon_assets.arrow.clone(),
            DEFAULT_BOLT_SIZE,
            damage,
            crit_damage,
            crit_chance,
            arrow_speed,
            direction,
            projectile_transform,
        ));

        audio
            .play(game_assets.crossbow_shoot.clone())
            .with_volume(game_settings.sound_volume);
    }
}
