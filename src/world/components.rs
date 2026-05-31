use bevy::prelude::*;
use serde::Deserialize;

#[derive(Component)]
pub struct Area;

#[derive(TypePath, Deserialize, Clone)]
pub enum AreaExit {
    Continue(String),
}

#[derive(Component)]
pub struct AreaExits(pub Vec<AreaExit>);

#[derive(Component)]
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

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct Inventory(pub Vec<Entity>);
