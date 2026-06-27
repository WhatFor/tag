use crate::prelude::*;
use bevy::prelude::*;

use crate::persistence::SAVE_FILE_KEY;
use crate::persistence::SAVE_FORMAT_VERSION;
use crate::persistence::data::SaveData;
use crate::persistence::data::SavedItem;
use crate::persistence::resources::SaveBackend;
use crate::player::components::CurrentArea;
use crate::prelude::Health;
use crate::sets::PlayingSet;
use crate::state::GameState;
use crate::world::bundles::default_player;
use crate::world::components::Area;
use crate::world::components::AreaId;
use crate::world::events::PlayerEnteredArea;

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

const START_AREA_ID: &str = "intro_01";

fn spawn_player(
    mut commands: Commands,
    areas: Query<(Entity, &AreaId), With<Area>>,
    store: Res<SaveBackend>,
) {
    info!("> Checking save state...");
    let save_data = store.read(SAVE_FILE_KEY);

    let start_data = match save_data {
        Ok(Some(save_data)) => {
            info!(save_data);

            let data: SaveData = ron::from_str(&save_data).expect("Failed to read save file.");

            data
        }
        _ => {
            info!("No save data!");

            SaveData {
                version: SAVE_FORMAT_VERSION,
                health: 100,
                current_area_id: START_AREA_ID.to_string(),
                last_checkpoint_area_id: START_AREA_ID.to_string(),
                path_taken: vec![],
                inventory: vec![
                    SavedItem {
                        count: 3,
                        item_id: String::from("potion"),
                    },
                    SavedItem {
                        count: 1,
                        item_id: String::from("iron_sword"),
                    },
                ],
            }
        }
    };

    info!("> Spawning player...");

    // Apply the player data, either default or from save file
    let start_area = areas
        .iter()
        .find(|(_, id)| ***id == start_data.current_area_id)
        .map(|(entity, _)| entity)
        .expect("Unable to find start area");

    let checkpoint_area = areas
        .iter()
        .find(|(_, id)| ***id == start_data.last_checkpoint_area_id)
        .map(|(entity, _)| entity)
        .expect("Unable to find last checkpoint area");

    let mut player = commands.spawn((
        default_player(),
        CurrentArea(start_area),
        LastCheckpointArea(checkpoint_area),
    ));

    // Must insert these after default spawn in order to replace specified components
    player.insert((
        Health(start_data.health),
        FullPathTaken(start_data.path_taken.clone()),
    ));

    for item in &start_data.inventory {
        player.give(item.item_id.clone(), item.count);
    }

    // Write out a save file, just to ensure one exists
    if let Ok(data) = ron::to_string(&start_data) {
        info!("Writing save data...");

        if let Err(e) = store.0.write(SAVE_FILE_KEY, &data) {
            warn!("Failed to write save data: {}", e);
        }
    }

    commands.trigger(PlayerEnteredArea(start_area));
}
