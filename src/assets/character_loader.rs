use std::collections::HashMap;

use bevy::log::*;
use bevy::{asset::LoadedFolder, prelude::*};

use crate::assets::character_asset_loader::{CharacterAssetLoader, CharacterData};
use crate::global::PendingAssets;
use crate::sets::PlayingSet;
use crate::state::GameState;

#[derive(Resource)]
pub struct CharacterFolder(pub Handle<LoadedFolder>);

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

        app.add_systems(OnEnter(GameState::Initialising), load_characters);

        app.add_systems(
            OnEnter(GameState::Playing),
            add_character_resource.in_set(PlayingSet::SpawnCharacters),
        );
    }
}

const CHARACTERS_ASSET_PATH: &str = "characters";

fn load_characters(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_tracker: ResMut<PendingAssets>,
) {
    info!("Beginning to load Character assets from folder...");
    let chars_handle = asset_server.load_folder(CHARACTERS_ASSET_PATH);
    asset_tracker.0.push(chars_handle.clone().untyped());
    commands.insert_resource(CharacterFolder(chars_handle));
}

fn add_character_resource(
    mut commands: Commands,
    char_data: Res<Assets<CharacterData>>,
    mut char_store: ResMut<CharacterStore>,
) {
    for (_, char_data) in char_data.iter() {
        info!("Storing Character {}...", char_data.id);

        char_store.0.insert(
            char_data.id.clone(),
            NarrativeCharacter {
                display_name: char_data.display_name.clone(),
            },
        );
    }

    commands.remove_resource::<CharacterFolder>();
}
