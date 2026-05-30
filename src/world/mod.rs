use bevy::log::*;
use bevy::prelude::*;

use crate::assets::AssetsPlugin;
use crate::components::Health;
use crate::player::components::{CurrentArea, Inventory, Player};
use crate::sets::PlayingSet;
use crate::state::GameState;
use crate::world::components::Area;
use crate::world::components::AreaId;
use crate::world::events::PlayerEnteredArea;

pub mod components;
pub mod events;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            spawn_player.in_set(PlayingSet::SpawnPlayer),
        );

        app.add_plugins(AssetsPlugin);
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

    let player_entity: Entity = commands
        .spawn((
            Player,
            //NameComponent(None),
            Health(100),
            Inventory(vec![]),
            CurrentArea(start_area),
        ))
        .id();

    commands.trigger(PlayerEnteredArea(start_area));
}
