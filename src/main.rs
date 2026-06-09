use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod prelude;
pub mod state;

mod assets;
mod components;
mod debug_tools;
mod game;
mod global;
mod main_menu;
mod pause_menu;
mod player;
mod sets;
mod ui;
mod world;

#[cfg(feature = "dev")]
use crate::debug_tools::DebugToolsPlugin;

use crate::assets::AssetsPlugin;
use crate::game::GameplayPlugin;
use crate::global::GlobalPlugin;
use crate::main_menu::MainMenuPlugin;
use crate::pause_menu::PauseMenuPlugin;
use crate::player::PlayerPlugin;
use crate::ui::UIPlugin;
use crate::world::WorldPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
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
    );

    #[cfg(feature = "dev")]
    app.add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new());

    app.add_plugins(AssetsPlugin)
        .add_plugins(GameplayPlugin)
        .add_plugins(GlobalPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(PauseMenuPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(PlayerPlugin);

    #[cfg(feature = "dev")]
    app.add_plugins(DebugToolsPlugin);

    app.run();
}
