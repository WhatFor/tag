use crate::prelude::*;
use bevy::prelude::*;

use crate::components::FullPathTaken;
use crate::persistence::data::{SaveData, SavedItem};
use crate::persistence::events::{SaveDeleted, SaveRequested};
use crate::persistence::resources::SaveBackend;
use crate::persistence::{SAVE_FILE_KEY, SAVE_FORMAT_VERSION};

pub struct PersistenceSavePlugin;

impl Plugin for PersistenceSavePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_save_requested);
        app.add_observer(on_save_deleted);
    }
}

fn on_save_requested(
    _: On<SaveRequested>,
    store: Res<SaveBackend>,
    player: Single<(&Health, &Inventory, &CurrentArea, &FullPathTaken), With<Player>>,
    areas: Query<&AreaId, With<Area>>,
    items: Query<(&ItemId, Option<&ItemStack>), With<Item>>,
) -> Result {
    info!("Saving game...");

    let (health, inventory, current_area, path_taken) = *player;

    let current_area_id = areas.get(current_area.entity())?.0.clone();

    let inventory = inventory
        .iter()
        .map(|&e| {
            let (id, stack) = items.get(e)?;

            Ok(SavedItem {
                item_id: (*id).clone(),
                count: stack.map_or(1, |s| **s),
            })
        })
        .collect::<Result<Vec<_>, BevyError>>()?;

    let save_data = SaveData {
        version: SAVE_FORMAT_VERSION,
        current_area_id: current_area_id,
        health: **health,
        inventory: inventory,
        path_taken: path_taken.0.clone(),
    };

    let save_data = ron::to_string(&save_data)?;

    if let Err(error) = store.write(SAVE_FILE_KEY, &save_data) {
        warn!("Save failed: {}", error);
    } else {
        info!("Game saved!");
    }

    Ok(())
}

fn on_save_deleted(_: On<SaveDeleted>, store: Res<SaveBackend>) -> Result {
    warn!("Deleting save...");
    store.0.clear(SAVE_FILE_KEY)?;
    info!("Save deleted!");

    Ok(())
}
