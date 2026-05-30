use bevy::prelude::*;

#[derive(Component)]
pub struct Area;

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
