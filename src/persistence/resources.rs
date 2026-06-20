use bevy::prelude::*;

use crate::persistence::store::SaveStore;

#[derive(Resource, Deref, DerefMut)]
pub struct SaveBackend(pub Box<dyn SaveStore>);
