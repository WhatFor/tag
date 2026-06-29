use crate::prelude::*;
use bevy::prelude::*;

pub fn default_player() -> impl Bundle {
    (
        Player,
        FullPathTaken(vec![]),
        Name::new("Player"),
        Health(100),
        Inventory(vec![]),
        DespawnOnEnter(GameState::MainMenu),
    )
}

pub fn item(item_def: &ItemDef) -> impl Bundle {
    (
        Item,
        ItemId(item_def.id.clone()),
        DisplayName(item_def.name.clone()),
        Description(item_def.description.clone()),
    )
}

pub fn item_stack(item_def: &ItemDef, count: u32) -> impl Bundle {
    debug_assert!(
        item_def.stackable,
        "tried to stack non-stackable item {}",
        item_def.id
    );

    (
        Item,
        ItemId(item_def.id.clone()),
        ItemStack(count),
        DisplayName(item_def.name.clone()),
        Description(item_def.description.clone()),
    )
}
