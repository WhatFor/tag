use crate::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::assets::area_loader::AreaLoaderPlugin;
use crate::assets::audio_loader::AudioSfxLoaderPlugin;
use crate::assets::character_loader::CharacterLoaderPlugin;
use crate::assets::enemy_icon_loader::EnemyIconLoaderPlugin;
use crate::assets::enemy_loader::EnemyLoaderPlugin;
use crate::assets::item_loader::ItemLoaderPlugin;
use crate::assets::texture_loader::TextureLoaderPlugin;
use crate::assets::ui_icon_loader::UiIconLoaderPlugin;

pub mod area_asset_loader;
pub mod area_loader;
pub mod audio_loader;
pub mod character_asset_loader;
pub mod character_loader;
pub mod enemy_asset_loader;
pub mod enemy_icon_loader;
pub mod enemy_loader;
pub mod item_asset_loader;
pub mod item_loader;
pub mod texture_loader;
pub mod ui_icon_loader;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Initialising).continue_to_state(GameState::MainMenu),
        );

        app.add_plugins(AreaLoaderPlugin);
        app.add_plugins(AudioSfxLoaderPlugin);
        app.add_plugins(CharacterLoaderPlugin);
        app.add_plugins(EnemyLoaderPlugin);
        app.add_plugins(ItemLoaderPlugin);
        app.add_plugins(UiIconLoaderPlugin);
        app.add_plugins(EnemyIconLoaderPlugin);
        app.add_plugins(TextureLoaderPlugin);
    }
}
