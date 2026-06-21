use bevy::prelude::*;

use crate::persistence::resources::SaveBackend;
use crate::persistence::save::PersistenceSavePlugin;
use crate::persistence::settings::PersistenceSettingsPlugin;
use crate::persistence::store::SaveStore;

pub struct PersistencePlugin;

pub mod data;
pub mod events;
pub mod resources;
pub mod settings;
pub mod store;

mod save;

pub const SAVE_FILE_KEY: &str = "SAVE_DATA";
pub const SAVE_FORMAT_VERSION: u32 = 1;

pub const SETTINGS_FILE_KEY: &str = "SETTINGS_DATA";
pub const SETTINGS_FORMAT_VERSION: u32 = 1;

impl Plugin for PersistencePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PersistenceSavePlugin);
        app.add_plugins(PersistenceSettingsPlugin);
        app.insert_resource(SaveBackend(default_store()));
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
    use crate::persistence::store::fs::FileSystemStorageStore;

    Box::new(FileSystemStorageStore::new())
}
