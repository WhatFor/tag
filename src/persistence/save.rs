use bevy::prelude::*;

use crate::persistence::events::SaveRequested;
use crate::persistence::store::SaveStore;

#[derive(Resource)]
pub struct SaveBackend(pub Box<dyn SaveStore>);

pub struct PersistenceSavePlugin;

impl Plugin for PersistenceSavePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_save_requested);
        app.insert_resource(SaveBackend(default_store()));
    }
}

fn on_save_requested(_: On<SaveRequested>, store: Res<SaveBackend>) {
    // TODO: Save. Probably want to pass to a SaveStore to abstract out the storage (WASM vs Local).
    info!("Saving game...");

    // TODO: filename? file content?
    if let Err(error) = store.0.write("SAVE_FILE", "SAVE_CONTENT") {
        warn!("Save failed: {}", error);
    }
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
    use crate::persistence::store::wasm::LocalStorageStore;

    Box::new(LocalStorageStore::new())
}
