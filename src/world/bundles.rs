use bevy::prelude::*;

use crate::{
    assets::item_loader::ItemDef,
    components::{Description, DisplayName, Health},
    player::components::Player,
    state::GameState,
    world::components::{Inventory, Item, ItemId, ItemStack},
};

pub fn default_player() -> impl Bundle {
    (
        Player,
        Name::new("Player"),
        Health(100),
        Inventory(vec![]),
        DespawnOnExit(GameState::Playing),
    )
}

pub fn item(def: &ItemDef) -> impl Bundle {
    (
        Item,
        ItemId(def.id.clone()),
        DisplayName(def.name.clone()),
        Description(def.description.clone()),
    )
}

pub fn item_stack(def: &ItemDef, count: u32) -> impl Bundle {
    debug_assert!(
        def.stackable,
        "tried to stack non-stackable item {}",
        def.id
    );

    (
        Item,
        ItemId(def.id.clone()),
        ItemStack(count),
        DisplayName(def.name.clone()),
        Description(def.description.clone()),
    )
}
