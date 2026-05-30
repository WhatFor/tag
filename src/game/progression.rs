use bevy::log::*;
use bevy::prelude::*;

use crate::game::events::PlayerContinued;
use crate::player::components::CurrentArea;
use crate::player::components::Player;
use crate::state::ExploringState;
use crate::ui::events::NarrationComplete;
use crate::world::components::Area;
use crate::world::components::AreaExit;
use crate::world::components::AreaExits;
use crate::world::components::AreaId;
use crate::world::events::PlayerEnteredArea;

pub struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_narration_completed);
        app.add_observer(on_player_continued);

        app.add_systems(
            OnEnter(ExploringState::AwaitingContinue),
            on_awaiting_continue,
        );
    }
}

fn on_narration_completed(
    _: On<NarrationComplete>,
    current_area: Single<&CurrentArea, With<Player>>,
    areas: Query<&AreaExits, With<Area>>,
    mut next_exploring_state: ResMut<NextState<ExploringState>>,
) {
    info!("Progression handling NarrationComplete event...");

    let Ok(current_area_exits) = areas.get(current_area.0) else {
        return;
    };

    match current_area_exits.0.first() {
        Some(AreaExit::Continue(next_room)) => {
            info!("Continuing on to room {}", next_room);

            // triggers show 'Press spacebar to continue...' prompt
            next_exploring_state.set(ExploringState::AwaitingContinue);
        }
        None => panic!("No exits!"),
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
            None => panic!("No exits!"),
        }
    };

    let Some((next_entity, _, _)) = areas.iter().find(|(_, id, _)| id.0 == next_room_id) else {
        return;
    };

    current_area.0 = next_entity;
    commands.trigger(PlayerEnteredArea(next_entity));
    next_exploring_state.set(ExploringState::Narrating);
}

fn on_awaiting_continue() {
    // show prompt!
}
