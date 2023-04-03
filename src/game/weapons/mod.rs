use bevy::prelude::*;

use crate::{utils::remove_all_with, GlobalState};

pub struct WeaponsPlugin;

pub mod area;
pub mod projectile;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(area::AreaPlugin)
            .add_plugin(projectile::ProjectilePlugin)
            .add_system(remove_all_with::<DamageMarker>.in_schedule(OnExit(GlobalState::InGame)));
    }
}

#[derive(Component)]
pub struct DamageMarker;
