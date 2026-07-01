use crate::prelude::*;
use bevy::prelude::*;

use crate::persistence::data::{CheckpointData, SaveData, SavedItem};
use crate::persistence::events::{SaveDeleted, SaveRequested};
use crate::persistence::resources::SaveBackend;
use crate::persistence::{CHECKPOINT_KEY_PREFIX, SAVE_FILE_KEY, SAVE_FORMAT_VERSION};
use crate::player::components::FullPathTaken;
use crate::player::components::Hardcore;

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
    player: Single<
        (
            &Health,
            &Gold,
            &Inventory,
            &CurrentArea,
            &LastCheckpointArea,
            &FullPathTaken,
            &Hardcore,
        ),
        With<Player>,
    >,
    areas: Query<&AreaId, With<Area>>,
    items: Query<(&ItemId, Option<&ItemStack>), With<Item>>,
) -> Result {
    info!("Saving game...");

    let (health, gold, inventory, current_area, last_checkpoint, path_taken, hardcore) = *player;

    let current_area_id = areas.get(current_area.entity())?.0.clone();
    let last_checkpoint_area_id = areas.get(last_checkpoint.entity())?.0.clone();

    let inventory = inventory
        .iter()
        .map(|&e| {
            let (id, stack) = items.get(e)?;

            Ok(SavedItem {
                item_id: (**id).clone(),
                count: stack.map_or(1, |s| **s),
            })
        })
        .collect::<Result<Vec<_>, BevyError>>()?;

    let save_data = {
        let save_data = SaveData {
            version: SAVE_FORMAT_VERSION,
            hardcore: hardcore.0,
            current_area_id: current_area_id.clone(),
            last_checkpoint_area_id: last_checkpoint_area_id,
            gold: **gold,
            health: **health,
            inventory: inventory,
            path_taken: path_taken.0.clone(),
        };

        let save_data_str = ron::to_string(&save_data)?;

        if let Err(error) = store.write(SAVE_FILE_KEY, &save_data_str) {
            warn!("Save failed: {}", error);
        } else {
            info!("Game saved!");
        }

        save_data
    };

    if save_data.last_checkpoint_area_id == current_area_id {
        // This is a checkpoint;
        // Store the current save state to support rolling back.
        let checkpoint = CheckpointData {
            checkpoint_area_id: save_data.last_checkpoint_area_id.clone(),
            save_at_checkpoint: save_data.clone(),
        };

        let checkpoint_data_str = ron::to_string(&checkpoint)?;

        // Key checkpoint on area_id
        let key = format!(
            "{}{}",
            CHECKPOINT_KEY_PREFIX, save_data.last_checkpoint_area_id
        );

        if let Err(error) = store.write(&key, &checkpoint_data_str) {
            warn!("Checkpoint failed: {}", error);
        } else {
            info!("Checkpoint saved!");
        }
    }

    Ok(())
}

fn on_save_deleted(_: On<SaveDeleted>, store: Res<SaveBackend>) -> Result {
    store.0.clear(SAVE_FILE_KEY)?;

    for key in store.keys()? {
        if key.starts_with(CHECKPOINT_KEY_PREFIX) {
            store.clear(&key)?;
        }
    }

    info!("Save deleted!");
    Ok(())
}
