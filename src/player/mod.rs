use crate::prelude::*;
use bevy::prelude::*;

pub mod components;
pub mod events;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_player_died);
    }
}

fn on_player_died(
    trigger: On<PlayerDied>,
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    commands.insert_resource(LastDeath(trigger.reason.clone()));
    next_game_state.set(GameState::Dead);
}
