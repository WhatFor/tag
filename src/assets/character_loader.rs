use std::collections::HashMap;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::assets::character_asset_loader::{CharacterAssetLoader, CharacterData};
use crate::sets::PlayingSet;
use crate::state::GameState;

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

///Loads Character.ron assets.
///
/// When the game launches, its's first in GameState::Initialising.
/// This triggers the 'load_characters' system.
/// On each Update, we run 'check_progress' to wait until all assets are loaded.
/// Once complete, the CharacterStore will be populated with all the NarrativeCharacters,
/// ready for use.
impl Plugin for CharacterLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<CharacterData>();
        app.init_asset_loader::<CharacterAssetLoader>();
        app.init_resource::<CharacterStore>();

        app.configure_loading_state(
            LoadingStateConfig::new(GameState::Initialising).load_collection::<CharacterAssets>(),
        );

        app.add_systems(
            OnEnter(GameState::Playing),
            add_character_resource.in_set(PlayingSet::SpawnCharacters),
        );
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

        info!("Spawning Character {}...", data.id);

        char_store.0.insert(
            data.id.clone(),
            NarrativeCharacter {
                display_name: data.display_name.clone(),
            },
        );
    }
}
