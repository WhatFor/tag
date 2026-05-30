use bevy::prelude::*;

use crate::{game::events::PlayerContinued, state::ExploringState};

pub mod components;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            wait_for_continue_input.run_if(in_state(ExploringState::AwaitingContinue)),
        );
    }
}

fn wait_for_continue_input(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        commands.trigger(PlayerContinued);
    }
}
