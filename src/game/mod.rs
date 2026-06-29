use bevy::prelude::*;

use crate::game::combat::CombatPlugin;
use crate::game::progression::ProgressionPlugin;

pub mod components;
pub mod events;
pub mod resources;

mod combat;
mod progression;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProgressionPlugin);
        app.add_plugins(CombatPlugin);
    }
}
