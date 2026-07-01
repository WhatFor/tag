use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Hardcore(pub bool);

#[derive(Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct CurrentArea(pub Entity);

#[derive(Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct LastCheckpointArea(pub Entity);

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
pub struct FullPathTaken(pub Vec<TakenPath>);

#[derive(Serialize, Deserialize, Reflect, Clone, PartialEq, Eq)]
pub struct TakenPath {
    pub area_id: String,
    pub choice_id: Option<String>, // None if no choices to be made
}

impl CurrentArea {
    pub fn entity(&self) -> Entity {
        self.0
    }
}
