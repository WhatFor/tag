use crate::prelude::*;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy_asset_loader::mapped::AssetFileStem;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct EnemyIconAssets {
    #[asset(paths("icons/enemies/small_rat.png",), collection(typed, mapped))]
    pub icons: HashMap<AssetFileStem, Handle<Image>>,
}

pub struct EnemyIconLoaderPlugin;

impl Plugin for EnemyIconLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(GameState::Initialising).load_collection::<EnemyIconAssets>(),
        );
    }
}
