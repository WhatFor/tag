use bevy::prelude::*;

use bevy::ui_widgets::ScrollbarPlugin;
use bevy::ui_widgets::SliderPlugin;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod prelude;
pub mod state;

mod assets;
mod audio;
mod game;
mod global;
mod introduction;
mod main_menu;
mod pause_menu;
mod persistence;
mod player;
mod settings_menu;
mod ui;
mod world;

mod components;
mod debug_tools;
mod sets;

#[cfg(feature = "dev")]
use crate::debug_tools::DebugToolsPlugin;

#[cfg(feature = "dev")]
use bevy::dev_tools::picking_debug::{DebugPickingMode, DebugPickingPlugin};

use crate::assets::AssetsPlugin;
use crate::audio::GameAudioPlugin;
use crate::game::GameplayPlugin;
use crate::global::GlobalPlugin;
use crate::introduction::IntroductionPlugin;
use crate::main_menu::MainMenuPlugin;
use crate::pause_menu::PauseMenuPlugin;
use crate::persistence::PersistencePlugin;
use crate::player::PlayerPlugin;
use crate::settings_menu::SettingsMenuPlugin;
use crate::ui::UIPlugin;
use crate::world::WorldPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                meta_check: bevy::asset::AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Window {
                    title: "TAG".to_string(),
                    fit_canvas_to_parent: true,
                    ..default()
                }
                .into(),
                ..default()
            }),
        ScrollbarPlugin,
        SliderPlugin,
    ));

    #[cfg(feature = "dev")]
    app.add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(DebugPickingPlugin);

    #[cfg(feature = "dev")]
    app.insert_resource(DebugPickingMode::Normal);

    app.add_plugins(AssetsPlugin)
        .add_plugins(GameAudioPlugin)
        .add_plugins(GameplayPlugin)
        .add_plugins(GlobalPlugin)
        .add_plugins(IntroductionPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(PauseMenuPlugin)
        .add_plugins(SettingsMenuPlugin)
        .add_plugins(PersistencePlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(PlayerPlugin);

    #[cfg(feature = "dev")]
    app.add_plugins(DebugToolsPlugin);

    app.run();
}
