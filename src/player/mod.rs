use bevy::prelude::*;

use crate::{game::events::PlayerContinued, global::PausableSystems, state::ExploringState};

pub mod components;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            wait_for_continue_input
                .run_if(in_state(ExploringState::AwaitingContinue))
                .in_set(PausableSystems),
        );
    }
}

fn wait_for_continue_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if keyboard.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left) {
        commands.trigger(PlayerContinued);
    }
}
