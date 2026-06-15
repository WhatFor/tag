use crate::persistence::data::{SaveData, SavedItem};
use crate::prelude::*;
use bevy::prelude::*;

use crate::persistence::events::{SaveDeleted, SaveRequested};
use crate::persistence::resources::SaveBackend;
use crate::persistence::store::SaveStore;
use crate::persistence::{SAVE_FILE_KEY, SAVE_FORMAT_VERSION};

pub struct PersistenceSavePlugin;

impl Plugin for PersistenceSavePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_save_requested);
        app.add_observer(on_save_deleted);
        app.insert_resource(SaveBackend(default_store()));
    }
}

fn on_save_requested(
    _: On<SaveRequested>,
    store: Res<SaveBackend>,
    player: Single<(&Health, &Inventory, &CurrentArea), With<Player>>,
    areas: Query<&AreaId, With<Area>>,
    items: Query<(&ItemId, Option<&ItemStack>), With<Item>>,
) -> Result {
    info!("Saving game...");

    let current_area_id = areas.get(player.2.0)?.0.clone();

    let inventory = player
        .1
        .0
        .iter()
        .map(|&e| {
            let (id, stack) = items.get(e)?;

            Ok(SavedItem {
                item_id: id.0.clone(),
                count: stack.map_or(1, |s| s.0),
            })
        })
        .collect::<Result<Vec<_>, BevyError>>()?;

    let save_data = SaveData {
        version: SAVE_FORMAT_VERSION,
        current_area_id: current_area_id,
        health: player.0.0,
        inventory: inventory,
    };

    let save_data = ron::to_string(&save_data)?;

    if let Err(error) = store.0.write(SAVE_FILE_KEY, &save_data) {
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

#[cfg(target_arch = "wasm32")]
fn default_store() -> Box<dyn SaveStore> {
    use crate::persistence::store::wasm::LocalStorageStore;

    Box::new(LocalStorageStore::new())
}

// TODO: Supporting non-web platforms in the design, but not implementing.
//       This will blow up atm but that's fine.
#[cfg(not(target_arch = "wasm32"))]
fn default_store() -> Box<dyn SaveStore> {
    use crate::persistence::store::fs::FileSystemStorageStore;

    Box::new(FileSystemStorageStore::new())
}
