use crate::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use bevy::platform::collections::HashMap;
use bevy_asset_loader::mapped::AssetFileStem;

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(paths("textures/panel_border.png"), collection(typed, mapped))]
    pub textures: HashMap<AssetFileStem, Handle<Image>>,
}

pub struct TextureLoaderPlugin;

impl Plugin for TextureLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(GameState::Initialising).load_collection::<TextureAssets>(),
        );
    }
}
