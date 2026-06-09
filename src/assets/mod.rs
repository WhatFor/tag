use crate::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::assets::area_loader::AreaLoaderPlugin;
use crate::assets::character_loader::CharacterLoaderPlugin;
use crate::assets::item_loader::ItemLoaderPlugin;

pub mod area_asset_loader;
pub mod area_loader;

pub mod character_asset_loader;
pub mod character_loader;

pub mod item_asset_loader;
pub mod item_loader;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Initialising).continue_to_state(GameState::MainMenu),
        );

        app.add_plugins(AreaLoaderPlugin);
        app.add_plugins(CharacterLoaderPlugin);
        app.add_plugins(ItemLoaderPlugin);
    }
}
