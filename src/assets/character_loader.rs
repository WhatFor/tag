use crate::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::assets::character_asset_loader::{CharacterAssetLoader, CharacterData};
use std::collections::HashMap;

#[derive(AssetCollection, Resource)]
pub struct CharacterAssets {
    #[asset(paths("characters/chair_daddy.char.ron"), collection(typed))]
    handles: Vec<Handle<CharacterData>>,
}

#[derive(Debug)]
pub struct NarrativeCharacter {
    pub display_name: String,
}

#[derive(Resource, Default, Debug)]
pub struct CharacterStore(pub HashMap<String, NarrativeCharacter>);

pub struct CharacterLoaderPlugin;

impl Plugin for CharacterLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<CharacterData>();
        app.init_asset_loader::<CharacterAssetLoader>();
        app.init_resource::<CharacterStore>();

        app.configure_loading_state(
            LoadingStateConfig::new(GameState::Initialising).load_collection::<CharacterAssets>(),
        );

        app.add_systems(OnExit(GameState::Initialising), add_character_resource);
    }
}

fn add_character_resource(
    chars: Res<CharacterAssets>,
    char_data: Res<Assets<CharacterData>>,
    mut char_store: ResMut<CharacterStore>,
) {
    for handle in &chars.handles {
        let Some(data) = char_data.get(handle) else {
            continue;
        };

        info!("Parsed Character {}...", data.id);

        char_store.0.insert(
            data.id.clone(),
            NarrativeCharacter {
                display_name: data.display_name.clone(),
            },
        );
    }
}
