use crate::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use bevy::platform::collections::HashMap;
use bevy_asset_loader::mapped::AssetFileStem;

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(
        paths(
            "audio/sfx/coins.wav",
            "audio/sfx/click.wav",
            "audio/sfx/sci_fi_thlonk.wav"
        ),
        collection(typed, mapped)
    )]
    pub sfx: HashMap<AssetFileStem, Handle<AudioSource>>,
    #[asset(paths("audio/music/placeholder.ogg"), collection(typed, mapped))]
    pub background: HashMap<AssetFileStem, Handle<AudioSource>>,
}

pub struct AudioSfxLoaderPlugin;

impl Plugin for AudioSfxLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(GameState::Initialising).load_collection::<AudioAssets>(),
        );
    }
}
