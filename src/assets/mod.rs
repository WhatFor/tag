use bevy::prelude::*;

use crate::assets::{area_loader::AreaLoaderPlugin, character_loader::CharacterLoaderPlugin};

pub mod area_asset_loader;
pub mod area_loader;
pub mod character_asset_loader;
pub mod character_loader;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AreaLoaderPlugin);
        app.add_plugins(CharacterLoaderPlugin);
    }
}
