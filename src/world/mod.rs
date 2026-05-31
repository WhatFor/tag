use bevy::prelude::*;

use crate::player::components::CurrentArea;
use crate::sets::PlayingSet;
use crate::state::GameState;
use crate::world::bundles::default_player;
use crate::world::components::Area;
use crate::world::components::AreaId;
use crate::world::events::PlayerEnteredArea;
use crate::world::inventory::GiveItemExt;

pub mod bundles;
pub mod components;
pub mod events;
pub mod inventory;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            spawn_player.in_set(PlayingSet::SpawnPlayer),
        );
    }
}

const START_AREA_ID: &str = "start_01";

fn spawn_player(mut commands: Commands, areas: Query<(Entity, &AreaId), With<Area>>) {
    info!("> Spawning player...");

    let start_area = areas
        .iter()
        .find(|(_, id)| id.0 == START_AREA_ID)
        .map(|(entity, _)| entity)
        .expect("Unable to find start area");

    commands
        .spawn((default_player(), CurrentArea(start_area)))
        .give("potion", 3)
        .give("iron_sword", 1);

    commands.trigger(PlayerEnteredArea(start_area));
}
