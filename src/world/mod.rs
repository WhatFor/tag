use bevy::prelude::*;

use crate::state::GameState;

mod components;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_game);
    }
}

fn setup_game(mut commands: Commands) {}
