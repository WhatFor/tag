use crate::prelude::*;
use bevy::prelude::*;

use crate::game::events::PlayerGameOver;
use crate::persistence::events::SaveRequested;

pub struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_player_continued);
        app.add_observer(on_player_chose);
        app.add_observer(on_player_game_over);
    }
}

fn on_player_continued(
    trigger: On<PlayerContinued>,
    mut commands: Commands,
    mut current_area: Single<&mut CurrentArea, With<Player>>,
    mut full_path_taken: Single<&mut FullPathTaken, With<Player>>,
    areas: Query<(Entity, &AreaId, &AreaExits), With<Area>>,
    mut next_exploring_state: ResMut<NextState<ExploringState>>,
) {
    let next_room_id = {
        let Ok((_, _, exits)) = areas.get(current_area.entity()) else {
            return;
        };

        match exits.first() {
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

    let Some((next_entity, _, _)) = areas.iter().find(|(_, id, _)| ***id == next_room_id) else {
        return;
    };

    // Set the AreaId on the Player's CurrentArea
    ***current_area = next_entity;

    // Add the Continue step to the Player's FullPathTaken
    full_path_taken.0.push(TakenPath {
        area_id: trigger.from.0.clone(),
        choice_id: None,
    });

    commands.trigger(PlayerEnteredArea(next_entity));
    commands.trigger(SaveRequested);
    next_exploring_state.set(ExploringState::PresentingContent);
}

fn on_player_chose(
    trigger: On<PlayerChose>,
    mut commands: Commands,
    mut current_area: Single<&mut CurrentArea, With<Player>>,
    mut full_path_taken: Single<&mut FullPathTaken, With<Player>>,
    areas: Query<(Entity, &AreaId, &AreaExits), With<Area>>,
    mut next_exploring_state: ResMut<NextState<ExploringState>>,
) {
    info!("Player chose to move to room {:?}", trigger);

    let Some((next_entity, _, _)) = areas.iter().find(|(_, id, _)| **id == trigger.to) else {
        warn!("Room {:?} not found!", trigger);
        return;
    };

    // Set the AreaId on the Player's CurrentArea
    ***current_area = next_entity;

    // Add the Choice to the Player's FullPathTaken
    full_path_taken.0.push(TakenPath {
        area_id: trigger.from.0.clone(),
        choice_id: Some(trigger.chosen_id.clone()),
    });

    commands.trigger(PlayerEnteredArea(next_entity));
    commands.trigger(SaveRequested);
    next_exploring_state.set(ExploringState::PresentingContent);
}

fn on_player_game_over(_: On<PlayerGameOver>, mut next_game_state: ResMut<NextState<GameState>>) {
    info!("Game over!");

    next_game_state.set(GameState::GameOver);
}
