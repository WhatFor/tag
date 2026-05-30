use bevy::prelude::*;

use crate::game::progression::ProgressionPlugin;

pub mod events;

mod progression;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProgressionPlugin);
    }
}
