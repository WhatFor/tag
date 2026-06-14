use bevy::prelude::*;
use serde::Deserialize;

#[derive(Component)]
pub struct Area;

#[derive(Deserialize, Clone)]
pub enum AreaExit {
    Continue(String),
    Choice(Vec<AreaExitOption>),
}

#[derive(Component)]
pub struct AreaExits(pub Vec<AreaExit>);

#[derive(Deserialize, Clone)]
pub struct AreaExitOption {
    pub to: AreaId,
    pub label: String,
}

#[derive(Component, Deserialize, Clone)]
pub struct AreaId(pub String);

#[derive(Component)]
pub struct AreaNarration {
    pub lines: Vec<String>,
}

#[derive(Component)]
pub struct AreaDialogue {
    pub character_id: Option<String>,
    pub lines: Vec<String>,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Item;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ItemId(pub String);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ItemStack(pub u32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Inventory(pub Vec<Entity>);
