use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{
    assets::{area_loader::AreaLoaderPlugin, character_loader::CharacterLoaderPlugin},
    state::GameState,
};

pub mod area_asset_loader;
pub mod area_loader;
pub mod character_asset_loader;
pub mod character_loader;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Initialising).continue_to_state(GameState::MainMenu),
        );

        app.add_plugins(AreaLoaderPlugin);
        app.add_plugins(CharacterLoaderPlugin);
    }
}
