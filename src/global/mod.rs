use bevy::prelude::*;

use crate::state::{GameState, PlayState};

pub struct GlobalPlugin;

impl Plugin for GlobalPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.add_sub_state::<PlayState>();

        app.add_systems(Startup, setup_global);
    }
}

fn setup_global(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    commands.spawn(Camera2d);
    next_state.set(GameState::MainMenu)
}
