use std::marker::PhantomData;

use bevy::prelude::*;

use super::{castle::CastleWall, enemies::Enemy, East, GameState, North, Side, South, West};

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyDamageEvent<North>>()
            .add_event::<EnemyDamageEvent<South>>()
            .add_event::<EnemyDamageEvent<West>>()
            .add_event::<EnemyDamageEvent<East>>()
            .add_event::<WallDamageEvent<North>>()
            .add_event::<WallDamageEvent<South>>()
            .add_event::<WallDamageEvent<West>>()
            .add_event::<WallDamageEvent<East>>()
            .add_systems(
                (
                    damage_enemy::<North>,
                    damage_enemy::<South>,
                    damage_enemy::<West>,
                    damage_enemy::<East>,
                    damage_wall::<North>,
                    damage_wall::<South>,
                    damage_wall::<West>,
                    damage_wall::<East>,
                )
                    .in_set(OnUpdate(GameState::InGame)),
            );
    }
}

/// Event to damage enemy
#[derive(Component)]
pub struct EnemyDamageEvent<S: Side> {
    pub target: Entity,
    pub damage: i32,
    _phantom: PhantomData<S>,
}

impl<S: Side> EnemyDamageEvent<S> {
    pub fn new(target: Entity, damage: i32) -> Self {
        Self {
            target,
            damage,
            _phantom: PhantomData,
        }
    }
}

/// Event to damage castle wall
#[derive(Component)]
pub struct WallDamageEvent<S: Side> {
    pub damage: i32,
    _phantom: PhantomData<S>,
}

impl<S: Side> WallDamageEvent<S> {
    pub fn new(damage: i32) -> Self {
        Self {
            damage,
            _phantom: PhantomData,
        }
    }
}

/// Damage enemies based on the side
fn damage_enemy<S: Side>(
    mut events: EventReader<EnemyDamageEvent<S>>,
    mut enemies: Query<&mut Enemy, With<S>>,
) {
    for event in events.iter() {
        if let Ok(mut enemy) = enemies.get_mut(event.target) {
            enemy.health -= event.damage;
        }
    }
}

/// Damage wall based on the side
fn damage_wall<S: Side>(
    mut events: EventReader<WallDamageEvent<S>>,
    mut wall: Query<&mut CastleWall, With<S>>,
) {
    let mut wall = wall.single_mut();
    for event in events.iter() {
        wall.health -= event.damage;
    }
}
