use crate::prelude::*;
use bevy::prelude::*;

use crate::game::events::PlayerGameOver;
use crate::persistence::events::SaveRequested;

pub struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_narration_completed);
        app.add_observer(on_dialogue_completed);
        app.add_observer(on_player_continued);
        app.add_observer(on_player_chose);
        app.add_observer(on_player_game_over);
    }
}

fn on_narration_completed(
    _: On<NarrationComplete>,
    current_area: Single<&CurrentArea, With<Player>>,
    areas: Query<(&AreaId, &AreaExits), With<Area>>,
    mut next_exploring_state: ResMut<NextState<ExploringState>>,
) {
    info!("Progression handling NarrationComplete event...");

    let Ok((area_id, current_area_exits)) = areas.get(current_area.0) else {
        return;
    };

    match current_area_exits.0.first() {
        Some(AreaExit::Continue(next_room)) => {
            info!("Continuing on to room {}", next_room);

            next_exploring_state.set(ExploringState::AwaitingContinue);
        }
        Some(AreaExit::Choice(_)) => {
            info!("Waiting for choice from room {}", area_id.0);
            next_exploring_state.set(ExploringState::AwaitingChoice);
        }
        Some(AreaExit::GameOver) => {
            info!("Waiting for GameOver continue {}", area_id.0);
            next_exploring_state.set(ExploringState::AwaitingGameOver);
        }
        None => {
            warn!("on_narration_completed: area '{}' has no exits", area_id.0);
        }
    }
}

fn on_dialogue_completed(
    _: On<DialogueComplete>,
    current_area: Single<&CurrentArea, With<Player>>,
    areas: Query<(&AreaId, &AreaExits), With<Area>>,
    mut next_exploring_state: ResMut<NextState<ExploringState>>,
) {
    info!("Progression handling DialogueComplete event...");

    let Ok((area_id, current_area_exits)) = areas.get(current_area.0) else {
        return;
    };

    match current_area_exits.0.first() {
        Some(AreaExit::Continue(next_room)) => {
            info!("Continuing on to room {}", next_room);

            next_exploring_state.set(ExploringState::AwaitingContinue);
        }
        Some(AreaExit::Choice(_)) => {
            info!("Waiting for choice from room {}", area_id.0);
            next_exploring_state.set(ExploringState::AwaitingChoice);
        }
        Some(AreaExit::GameOver) => {
            info!("Waiting for GameOver continue {}", area_id.0);
            next_exploring_state.set(ExploringState::AwaitingGameOver);
        }
        None => {
            warn!("on_dialogue_completed: area '{}' has no exits", area_id.0);
        }
    }
}

fn on_player_continued(
    _: On<PlayerContinued>,
    mut commands: Commands,
    mut current_area: Single<&mut CurrentArea, With<Player>>,
    areas: Query<(Entity, &AreaId, &AreaExits), With<Area>>,
    mut next_exploring_state: ResMut<NextState<ExploringState>>,
) {
    let next_room_id = {
        let Ok((_, _, exits)) = areas.get(current_area.0) else {
            return;
        };

        match exits.0.first() {
            Some(AreaExit::Continue(id)) => id.clone(),
            Some(AreaExit::Choice(_)) => {
                // TODO: shouldn't be possible
                todo!();
            }
            Some(AreaExit::GameOver) => {
                // TODO: shouldn't be possible
                todo!()
            }
            None => panic!("No exits!"),
        }
    };

    let Some((next_entity, _, _)) = areas.iter().find(|(_, id, _)| id.0 == next_room_id) else {
        return;
    };

    current_area.0 = next_entity;
    commands.trigger(PlayerEnteredArea(next_entity));
    commands.trigger(SaveRequested);
    next_exploring_state.set(ExploringState::PresentingContent);
}

fn on_player_chose(
    trigger: On<PlayerChose>,
    mut commands: Commands,
    mut current_area: Single<&mut CurrentArea, With<Player>>,
    areas: Query<(Entity, &AreaId, &AreaExits), With<Area>>,
    mut next_exploring_state: ResMut<NextState<ExploringState>>,
) {
    info!("Player chose to move to room {}", trigger.0.0);

    let Some((next_entity, _, _)) = areas.iter().find(|(_, id, _)| id.0 == trigger.0.0) else {
        warn!("Room {} not found!", trigger.0.0);
        return;
    };

    current_area.0 = next_entity;
    commands.trigger(PlayerEnteredArea(next_entity));
    commands.trigger(SaveRequested);
    next_exploring_state.set(ExploringState::PresentingContent);
}

fn on_player_game_over(_: On<PlayerGameOver>, mut next_game_state: ResMut<NextState<GameState>>) {
    info!("Game over!");

    next_game_state.set(GameState::GameOver);
}
