use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{utils::remove_all_with, GlobalState};

pub struct WeaponsPlugin;

pub mod area;
pub mod projectile;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, WeaponsAssets>(GlobalState::AssetLoading)
            .add_plugin(area::AreaPlugin)
            .add_plugin(projectile::ProjectilePlugin)
            .add_system(remove_all_with::<DamageMarker>.in_schedule(OnExit(GlobalState::InGame)));
    }
}

#[derive(AssetCollection, Resource)]
struct WeaponsAssets {
    #[asset(path = "sprites/arrow.png")]
    arrow: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 3, rows = 1,))]
    #[asset(path = "sprites/fire.png")]
    pub fire: Handle<TextureAtlas>,
}

#[derive(Component)]
pub struct DamageMarker;
