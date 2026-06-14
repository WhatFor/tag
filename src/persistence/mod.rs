use bevy::prelude::*;

use crate::persistence::save::PersistenceSavePlugin;

pub struct PersistencePlugin;

pub mod events;
pub mod store;

mod data;
mod save;

impl Plugin for PersistencePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PersistenceSavePlugin);
    }
}
