use bevy::prelude::*;
use serde::Deserialize;

use crate::prelude::TakenPath;

#[derive(Component)]
pub struct Area;

#[derive(Component)]
pub struct CheckpointArea;

#[derive(Deserialize, Clone)]
pub enum AreaExit {
    Continue(String),
    Choice(Vec<AreaExitOption>),
    GameOver(String),
}

#[derive(Component, Deref)]
pub struct AreaExits(pub Vec<AreaExit>);

#[derive(Deserialize, Clone)]
pub struct AreaExitOption {
    pub id: String,
    pub to: AreaId,
    pub label: String,
    pub requires: Option<Vec<AreaExitRequirement>>,
}

#[derive(Deserialize, Clone)]
pub enum AreaExitRequirement {
    // Dictates that a certain AreaId must have been passed through
    // and, optionally, the choice that had to have been selected.
    TookPath(TakenPath),
    // Requires the player has at least a certain number of
    // the specified item.
    HasItem(ItemId, u32),
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
    Combat {
        start_lines: Vec<String>,
        enemy_ids: Vec<String>,
        win_lines: Vec<String>,
        lose_lines: Vec<String>,
    },
}

#[derive(Clone, Debug, Deserialize, Reflect, Deref, PartialEq, Eq, Hash)]
pub struct ItemId(pub String);

impl Into<String> for ItemId {
    fn into(self) -> String {
        self.0
    }
}

impl Into<ItemId> for String {
    fn into(self) -> ItemId {
        ItemId(self)
    }
}

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct ItemStack {
    pub item_id: ItemId,
    pub count: u32,
}

#[derive(Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct Inventory(pub Vec<ItemStack>);
