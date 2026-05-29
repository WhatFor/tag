use bevy::prelude::*;

mod assets;
mod components;
mod global;
mod main_menu;
mod player;
mod state;
mod world;

use crate::global::GlobalPlugin;
use crate::main_menu::MainMenuPlugin;
use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GlobalPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
