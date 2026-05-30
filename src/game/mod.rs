use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{game::progression::ProgressionPlugin, global::Pause, state::GameState};

pub mod events;

mod progression;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProgressionPlugin);

        app.add_systems(
            Update,
            pause.run_if(
                in_state(Pause(false))
                    .and(in_state(GameState::Playing))
                    .and(input_just_pressed(KeyCode::Escape)),
            ),
        );

        app.add_systems(
            Update,
            unpause.run_if(
                in_state(Pause(true))
                    .and(in_state(GameState::Playing))
                    .and(input_just_pressed(KeyCode::Escape)),
            ),
        );
    }
}

fn unpause(mut next_pause_state: ResMut<NextState<Pause>>) {
    next_pause_state.set(Pause(false));
}

fn pause(mut next_pause_state: ResMut<NextState<Pause>>) {
    next_pause_state.set(Pause(true));
}
