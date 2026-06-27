use bevy::prelude::*;

use crate::game::progression::ProgressionPlugin;

pub mod events;
pub mod resources;

mod progression;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProgressionPlugin);
    }
}
