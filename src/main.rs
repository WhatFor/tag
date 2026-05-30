use bevy::prelude::*;

pub mod state;

mod assets;
mod components;
mod game;
mod global;
mod main_menu;
mod player;
mod sets;
mod ui;
mod world;

use crate::game::GameplayPlugin;
use crate::global::GlobalPlugin;
use crate::main_menu::MainMenuPlugin;
use crate::player::PlayerPlugin;
use crate::ui::UIPlugin;
use crate::world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameplayPlugin)
        .add_plugins(GlobalPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
