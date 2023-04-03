use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::game::{
    animation::AnimationBundle, damage::EnemyDamageEvent, East, GameState, North, Side, South, West,
};

use super::{DamageMarker, WeaponsAssets};

const DEFAULT_AREA_SIZE: f32 = 64.0;
const DEFAULT_AREA_DAMAGE: i32 = 10;
const DEFAULT_AREA_ATTACK_SPEED: f32 = 0.5;
const DEFAULT_AREA_LIFESPAN: f32 = 5.0;

const DEFAULT_CATAPULTE_MIN_RANGE: f32 = 100.0;
const DEFAULT_CATAPULTE_RANGE: f32 = 400.0;
const DEFAULT_CATAPULTE_ATTACK_SPEED: f32 = 3.0;

pub struct AreaPlugin;

impl Plugin for AreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                catapulte_attack::<North>,
                catapulte_attack::<South>,
                catapulte_attack::<West>,
                catapulte_attack::<East>,
                area_update::<North>,
                area_update::<South>,
                area_update::<West>,
                area_update::<East>,
            )
                .in_set(OnUpdate(GameState::InGame)),
        );
    }
}

#[derive(Component)]
pub struct Catapulte {
    damage: i32,
    range: f32,
    area_size: f32,
    area_attack_speed: f32,
    attack_timer: Timer,
}

impl Default for Catapulte {
    fn default() -> Self {
        Self {
            damage: DEFAULT_AREA_DAMAGE,
            range: DEFAULT_CATAPULTE_RANGE,
            area_size: DEFAULT_AREA_SIZE,
            area_attack_speed: DEFAULT_AREA_ATTACK_SPEED,
            attack_timer: Timer::from_seconds(DEFAULT_CATAPULTE_ATTACK_SPEED, TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct Area {
    size: f32,
    damage: i32,
    attack_timer: Timer,
    lifespan: Timer,
}

#[derive(Bundle)]
pub struct AreaBundle<S: Side> {
    #[bundle]
    animation_bundle: AnimationBundle,
    area: Area,
    side: S,
    marker: DamageMarker,
}

impl<S: Side> AreaBundle<S> {
    fn new(
        texture_atlas: Handle<TextureAtlas>,
        size: f32,
        damage: i32,
        attack_speed: f32,
        position: Vec3,
    ) -> Self {
        Self {
            animation_bundle: AnimationBundle::new_with_size(
                texture_atlas,
                Vec2::new(size, size),
                2,
                12.0,
                position,
            ),
            area: Area {
                size,
                damage,
                attack_timer: Timer::from_seconds(attack_speed, TimerMode::Repeating),
                lifespan: Timer::from_seconds(DEFAULT_AREA_LIFESPAN, TimerMode::Once),
            },
            side: S::default(),
            marker: DamageMarker,
        }
    }
}

fn catapulte_attack<S: Side>(
    time: Res<Time>,
    weapon_assets: Res<WeaponsAssets>,
    mut commands: Commands,
    mut catapultes: Query<(&Transform, &mut Catapulte), With<S>>,
) {
    for (transform, mut catapulte) in catapultes.iter_mut() {
        if !catapulte.attack_timer.tick(time.delta()).finished() {
            continue;
        }

        let mut rng = rand::thread_rng();
        // each side is 60 degrees in size.
        // S::direction gives a line directly at the center of the side
        let angle = rng.gen_range(-30.0..30.0);
        let distance = rng.gen_range(DEFAULT_CATAPULTE_MIN_RANGE..catapulte.range);

        // convert angle to radians
        let direction = Vec2::from_angle(angle / 360.0 * std::f32::consts::PI).rotate(S::DIRECTION);

        let mut area_position = transform.translation;
        area_position += (direction * distance).extend(0.0);

        commands.spawn(AreaBundle::<S>::new(
            weapon_assets.fire.clone(),
            catapulte.area_size,
            catapulte.damage,
            catapulte.area_attack_speed,
            area_position,
        ));
    }
}

fn area_update<S: Side>(
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
    mut areas: Query<(Entity, &Transform, &mut Area), With<S>>,
    mut damage_event: EventWriter<EnemyDamageEvent<S>>,
) {
    for (area_entity, area_transform, mut area) in areas.iter_mut() {
        if area.lifespan.tick(time.delta()).finished() {
            commands.entity(area_entity).despawn();
        } else {
            if !area.attack_timer.tick(time.delta()).finished() {
                continue;
            }

            let callback = |e| {
                damage_event.send(EnemyDamageEvent::new(e, area.damage));
                true
            };

            rapier_context.intersections_with_shape(
                area_transform.translation.truncate(),
                0.0,
                &Collider::ball(area.size),
                QueryFilter::only_dynamic(),
                callback,
            );
        }
    }
}
