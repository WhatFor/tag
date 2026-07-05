use crate::prelude::*;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy_asset_loader::mapped::AssetFileStem;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct IconAssets {
    #[asset(
        paths(
            "icons/ui/inventory.png",
            "icons/ui/x.png",
            "icons/ui/strength.png",
            "icons/ui/agility.png",
            "icons/ui/intelligence.png",
            "icons/ui/speed.png",
            "icons/ui/armour.png",
        ),
        collection(typed, mapped)
    )]
    pub icons: HashMap<AssetFileStem, Handle<Image>>,
}

pub struct IconLoaderPlugin;

impl Plugin for IconLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(GameState::Initialising).load_collection::<IconAssets>(),
        );
    }
}
