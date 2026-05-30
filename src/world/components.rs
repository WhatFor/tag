use bevy::prelude::*;
use serde::Deserialize;

#[derive(Component)]
pub struct Area;

#[derive(Asset, TypePath, Deserialize, Clone)]
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

pub struct DialogueLine {
    pub character_id: String,
    pub line: String,
}

#[derive(Component)]
pub struct AreaDialogue {
    pub lines: Vec<DialogueLine>,
}

#[derive(Component)]
pub struct Item;
