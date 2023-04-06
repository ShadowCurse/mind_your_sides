use std::marker::PhantomData;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    game::{damage::area::DamageAreaBundle, East, GameState, North, Side, South, West},
    utils::remove_all_with,
    GlobalState,
};

use super::{GlobalWeaponBuffs, WeaponsAssets};

const DEFAULT_AREA_SIZE: f32 = 40.0;
const DEFAULT_AREA_DAMAGE: i32 = 10;
const DEFAULT_AREA_ATTACK_SPEED: f32 = 0.5;
const DEFAULT_AREA_LIFESPAN: f32 = 3.0;

const DEFAULT_MOLOTOV_MIN_RANGE: f32 = 150.0;
const DEFAULT_MOLOTOV_RANGE: f32 = 300.0;
const DEFAULT_MOLOTOV_ATTACK_SPEED: f32 = 3.0;

pub struct MolotovPlugin;

impl Plugin for MolotovPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GlobalState::InGame)))
            .add_systems(
                (
                    molotov_attack::<North>,
                    molotov_attack::<South>,
                    molotov_attack::<West>,
                    molotov_attack::<East>,
                )
                    .in_set(OnUpdate(GameState::InGame)),
            )
            .add_system(remove_all_with::<MolotovMarker>.in_schedule(OnExit(GlobalState::InGame)));
    }
}

#[derive(Component)]
pub struct MolotovMarker;

#[derive(Default, Resource)]
pub struct MolotovBuffs<S: Side> {
    pub damage: f32,
    pub damage_flat: i32,
    pub crit_damage: f32,
    pub crit_chance: f32,
    pub area_size: f32,
    pub attack_speed: f32,
    pub area_attack_speed: f32,
    pub area_lifespan: f32,
    _phatom: PhantomData<S>,
}

#[derive(Component)]
pub struct Molotov<S: Side> {
    damage: i32,
    range: f32,
    area_size: f32,
    area_attack_speed: f32,
    attack_timer: Timer,
    _phatom: PhantomData<S>,
}

impl<S: Side> Default for Molotov<S> {
    fn default() -> Self {
        Self {
            damage: DEFAULT_AREA_DAMAGE,
            range: DEFAULT_MOLOTOV_RANGE,
            area_size: DEFAULT_AREA_SIZE,
            area_attack_speed: DEFAULT_AREA_ATTACK_SPEED,
            attack_timer: Timer::from_seconds(DEFAULT_MOLOTOV_ATTACK_SPEED, TimerMode::Repeating),
            _phatom: PhantomData,
        }
    }
}

#[derive(Bundle)]
pub struct MolotovBundle<S: Side> {
    crossbow: Molotov<S>,
    marker: MolotovMarker,
}

impl<S: Side> Default for MolotovBundle<S> {
    fn default() -> Self {
        Self {
            crossbow: Default::default(),
            marker: MolotovMarker,
        }
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(MolotovBuffs::<North>::default());
    commands.insert_resource(MolotovBuffs::<South>::default());
    commands.insert_resource(MolotovBuffs::<West>::default());
    commands.insert_resource(MolotovBuffs::<East>::default());
}

fn molotov_attack<S: Side>(
    time: Res<Time>,
    weapon_assets: Res<WeaponsAssets>,
    molotov_buffs: Res<MolotovBuffs<S>>,
    global_weapons_buffs: Res<GlobalWeaponBuffs>,
    mut commands: Commands,
    mut molotovs: Query<(&Transform, &mut Molotov<S>)>,
) {
    for (transform, mut molotov) in molotovs.iter_mut() {
        if !molotov.attack_timer.tick(time.delta()).finished() {
            continue;
        }

        molotov.attack_timer = Timer::from_seconds(
            DEFAULT_MOLOTOV_ATTACK_SPEED * (1.0 + molotov_buffs.attack_speed),
            TimerMode::Repeating,
        );

        let mut rng = rand::thread_rng();
        // each side is 60 degrees in size.
        // S::direction gives a line directly at the center of the side
        let angle = rng.gen_range(-30.0..30.0);
        let distance = rng.gen_range(DEFAULT_MOLOTOV_MIN_RANGE..molotov.range);

        // convert angle to radians
        let direction = Vec2::from_angle(angle / 360.0 * std::f32::consts::PI).rotate(S::DIRECTION);

        let mut area_position = transform.translation;
        area_position += (direction * distance).extend(0.0);

        let damage =
            ((molotov.damage + molotov_buffs.damage_flat + global_weapons_buffs.damage_flat) as f32
                * (1.0 + molotov_buffs.damage + global_weapons_buffs.damage)) as i32;
        let area_size = molotov.area_size * (1.0 + molotov_buffs.area_size);
        let area_attack_speed = molotov.area_attack_speed * (1.0 + molotov_buffs.area_attack_speed);
        let area_lifespan = DEFAULT_AREA_LIFESPAN * (1.0 + molotov_buffs.area_lifespan);
        let crit_chance = molotov_buffs.crit_chance + global_weapons_buffs.crit_chance;
        let crit_damage = (molotov.damage as f32
            * (1.0 + molotov_buffs.crit_damage + global_weapons_buffs.crit_damage))
            as i32;

        commands.spawn(DamageAreaBundle::<S>::new(
            weapon_assets.fire.clone(),
            area_size,
            damage,
            crit_damage,
            crit_chance,
            area_attack_speed,
            area_lifespan,
            area_position,
        ));
    }
}
