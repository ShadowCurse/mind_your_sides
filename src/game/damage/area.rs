use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    game::{
        animation::AnimationBundle, damage::EnemyDamageEvent, East, GameState, North, Side, South,
        West,
    },
    utils::remove_all_with,
    GlobalState,
};

pub struct AreaPlugin;

impl Plugin for AreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                damage_area_update::<North>,
                damage_area_update::<South>,
                damage_area_update::<West>,
                damage_area_update::<East>,
            )
                .in_set(OnUpdate(GameState::InGame)),
        )
        .add_system(remove_all_with::<DamageAreaMarker>.in_schedule(OnExit(GlobalState::InGame)));
    }
}

#[derive(Component)]
pub struct DamageAreaMarker;

#[derive(Clone, Component)]
pub struct DamageArea<S: Side> {
    size: f32,
    damage: i32,
    crit_damage: i32,
    crit_chance: f32,
    attack_timer: Timer,
    lifespan: Timer,
    _phatom: PhantomData<S>,
}

impl<S: Side> DamageArea<S> {
    pub fn new(
        size: f32,
        damage: i32,
        crit_damage: i32,
        crit_chance: f32,
        attack_speed: f32,
        lifespan: f32,
    ) -> Self {
        Self {
            size,
            damage,
            crit_damage,
            crit_chance,
            attack_timer: Timer::from_seconds(attack_speed, TimerMode::Repeating),
            lifespan: Timer::from_seconds(lifespan, TimerMode::Once),
            _phatom: PhantomData,
        }
    }
}

#[derive(Bundle)]
pub struct DamageAreaBundle<S: Side> {
    #[bundle]
    animation_bundle: AnimationBundle,
    area: DamageArea<S>,
    marker: DamageAreaMarker,
}

impl<S: Side> DamageAreaBundle<S> {
    pub fn new(texture_atlas: Handle<TextureAtlas>, position: Vec3, area: DamageArea<S>) -> Self {
        Self {
            animation_bundle: AnimationBundle::new_with_size(
                texture_atlas,
                Vec2::new(area.size * 2.0, area.size * 2.0),
                2,
                12.0,
                position,
            ),
            area,
            marker: DamageAreaMarker,
        }
    }
}

fn damage_area_update<S: Side>(
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
    mut areas: Query<(Entity, &Transform, &mut DamageArea<S>)>,
    mut damage_event: EventWriter<EnemyDamageEvent<S>>,
) {
    let mut rng = rand::thread_rng();
    for (area_entity, area_transform, mut area) in areas.iter_mut() {
        if area.lifespan.tick(time.delta()).finished() {
            commands.entity(area_entity).despawn();
        } else {
            if !area.attack_timer.tick(time.delta()).finished() {
                continue;
            }

            let callback = |e| {
                let (damage, was_crit) = if rng.gen_range(0.0..1.0) < area.crit_chance {
                    (area.crit_damage, true)
                } else {
                    (area.damage, false)
                };

                damage_event.send(EnemyDamageEvent::new(e, damage, was_crit));
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
