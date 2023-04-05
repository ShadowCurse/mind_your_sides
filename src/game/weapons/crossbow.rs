use std::marker::PhantomData;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    game::{
        damage::projectile::ProjectileBundle, enemies::Enemy, East, GameState, North, Side, South,
        West,
    },
    utils::remove_all_with,
    GlobalState,
};

use super::{GlobalWeaponBuffs, WeaponsAssets};

const DEFAULT_BOLT_SIZE: f32 = 3.0;
const DEFAULT_BOLT_DAMAGE: i32 = 20;
const DEFAULT_BOLT_SPEED: f32 = 200.0;

const DEFAULT_CROSSBOW_RANGE: f32 = 200.0;
const DEFAULT_CROSSBOW_ATTACK_SPEED: f32 = 1.0;
/// Offsets arrow spawn point in the enemy direction
const DEFAULT_BOLT_SPAWN_OFFSET: f32 = 30.0;

pub struct CrossbowPlugin;

impl Plugin for CrossbowPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::InGame)))
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

#[derive(Resource)]
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

impl<S: Side> Default for CrossbowBuffs<S> {
    fn default() -> Self {
        Self {
            damage: 1.0,
            damage_flat: 0,
            crit_damage: 2.0,
            crit_chance: 10.0,
            range: 1.0,
            attack_speed: 1.0,
            arrow_speed: 1.0,
            _phantom: PhantomData,
        }
    }
}

#[derive(Component)]
pub struct Crossbow<S: Side> {
    damage: i32,
    range: f32,
    arrow_speed: f32,
    attack_timer: Timer,
    _phantom: PhantomData<S>,
}

impl<S: Side> Default for Crossbow<S> {
    fn default() -> Self {
        Self {
            damage: DEFAULT_BOLT_DAMAGE,
            range: DEFAULT_CROSSBOW_RANGE,
            arrow_speed: DEFAULT_BOLT_SPEED,
            attack_timer: Timer::from_seconds(DEFAULT_CROSSBOW_ATTACK_SPEED, TimerMode::Repeating),
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
            DEFAULT_CROSSBOW_ATTACK_SPEED * crossbow_buffs.attack_speed,
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
        let range = crossbow.range * crossbow_buffs.range;
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

        let mut damage =
            ((crossbow.damage + crossbow_buffs.damage_flat + global_weapons_buffs.damage_flat)
                as f32
                * (crossbow_buffs.damage + global_weapons_buffs.damage)) as i32;
        let arrow_speed = crossbow.arrow_speed * crossbow_buffs.arrow_speed;
        let crit_chance = crossbow_buffs.crit_chance + global_weapons_buffs.crit_chance;
        let crit_damage = crossbow_buffs.crit_damage + global_weapons_buffs.crit_damage;

        if rand::thread_rng().gen_range(0.0..100.0) < crit_chance {
            damage = (damage as f32 * crit_damage) as i32;
        }

        commands.spawn(ProjectileBundle::<S>::new(
            weapon_assets.arrow.clone(),
            DEFAULT_BOLT_SIZE,
            damage,
            arrow_speed,
            direction,
            projectile_transform,
        ));
    }
}
