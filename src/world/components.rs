use bevy::prelude::*;
use serde::Deserialize;

#[derive(Component)]
pub struct Area;

#[derive(Deserialize, Clone)]
pub enum AreaExit {
    Continue(String),
    Choice(Vec<AreaExitOption>),
    GameOver,
}

#[derive(Component, Deref)]
pub struct AreaExits(pub Vec<AreaExit>);

#[derive(Deserialize, Clone)]
pub struct AreaExitOption {
    pub to: AreaId,
    pub label: String,
}

#[derive(Component, Deserialize, Clone, Debug, Deref, PartialEq, Eq)]
pub struct AreaId(pub String);

#[derive(Component, Deserialize, Clone)]
pub enum AreaContent {
    Dialogue {
        character_id: Option<String>,
        lines: Vec<String>,
    },
    Narration {
        lines: Vec<String>,
    },
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Item;

#[derive(Component, Reflect, Deref, PartialEq, Eq)]
#[reflect(Component)]
pub struct ItemId(pub String);

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
pub struct ItemStack(pub u32);

#[derive(Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct Inventory(pub Vec<Entity>);
