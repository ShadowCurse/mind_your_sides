use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::GlobalState;

use super::GameState;

pub struct WeaponsPlugin;

pub mod crossbow;
pub mod molotov;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, WeaponsAssets>(GlobalState::AssetLoading)
            .add_system(setup.in_schedule(OnEnter(GameState::InGame)))
            .add_plugin(crossbow::CrossbowPlugin)
            .add_plugin(molotov::MolotovPlugin);
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

#[derive(Default, Resource)]
pub struct GlobalWeaponBuffs {
    pub damage: f32,
    pub damage_flat: i32,
    pub crit_damage: f32,
    pub crit_chance: f32,
}

fn setup(mut commands: Commands) {
    commands.insert_resource(GlobalWeaponBuffs::default());
}
