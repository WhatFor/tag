use bevy::prelude::*;

use crate::game::combat::CombatPlugin;
use crate::game::economy::EconomyPlugin;
use crate::game::progression::ProgressionPlugin;
use crate::game::resources::HardcoreMode;

pub mod components;
pub mod events;
pub mod resources;

mod combat;
mod economy;
mod progression;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HardcoreMode(false));
        app.add_plugins(ProgressionPlugin);
        app.add_plugins(CombatPlugin);
        app.add_plugins(EconomyPlugin);
    }
}
