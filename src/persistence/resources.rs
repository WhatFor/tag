use bevy::prelude::*;

use crate::persistence::store::SaveStore;

#[derive(Resource)]
pub struct SaveBackend(pub Box<dyn SaveStore>);
