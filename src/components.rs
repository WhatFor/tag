use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
pub struct DisplayName(pub String);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Description(pub String);

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
pub struct Health(pub usize);

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
pub struct FullPathTaken(pub Vec<TakenPath>);

#[derive(Serialize, Deserialize, Reflect, Clone)]
pub struct TakenPath {
    pub area_id: String,
    pub choice_id: Option<String>, // None if no choices to be made
}
