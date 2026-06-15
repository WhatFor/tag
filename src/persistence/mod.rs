use bevy::prelude::*;

use crate::persistence::save::PersistenceSavePlugin;

pub struct PersistencePlugin;

pub mod data;
pub mod events;
pub mod resources;
pub mod store;

mod save;

pub const SAVE_FILE_KEY: &str = "SAVE_DATA";
pub const SAVE_FORMAT_VERSION: u32 = 1;

impl Plugin for PersistencePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PersistenceSavePlugin);
    }
}
