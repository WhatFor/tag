use crate::{prelude::*, world::inventory::TakeItemExt};
use bevy::prelude::*;

pub struct EquipItem {
    pub recipient: Entity,
    pub item_id: ItemId,
}

pub struct EquipFromInventory {
    pub recipient: Entity,
    pub item_id: ItemId,
}

pub trait EquipItemExt {
    fn equip(&mut self, item_id: impl Into<ItemId>) -> &mut Self;
    fn equip_from_inventory(&mut self, item_id: impl Into<ItemId>) -> &mut Self;
}

impl Command for EquipItem {
    fn apply(self, world: &mut World) {
        let Some(def) = world.resource::<ItemStore>().get(&self.item_id.0).cloned() else {
            warn!("Equip failed to find item. ID: {:?}", self.item_id);
            return;
        };

        let Some(slot) = def.slot else {
            warn!("Item is not equippable. ID: {:?}", self.item_id);
            return;
        };

        let mut recipient = world.entity_mut(self.recipient);

        match recipient.get_mut::<Equipment>() {
            Some(mut equipment) => {
                if let Some(_previous) = equipment.insert(slot, ItemId(def.id.clone())) {
                    // If there was something already equipped, move
                    // it back to inventory
                    world.commands().entity(self.recipient).give(def.id, 1);
                }
            }
            None => {
                panic!("Entity doesn't have an inventory.");
            }
        }
    }
}

impl Command for EquipFromInventory {
    fn apply(self, world: &mut World) -> () {
        let mut commands = world.commands();
        let mut entity_commands = commands.entity(self.recipient);
        entity_commands.take(self.item_id.clone(), 1);
        entity_commands.equip(self.item_id);
    }
}

impl EquipItemExt for EntityCommands<'_> {
    fn equip(&mut self, item_id: impl Into<ItemId>) -> &mut Self {
        let recipient = self.id();

        self.commands().queue(EquipItem {
            item_id: item_id.into(),
            recipient,
        });

        self
    }

    fn equip_from_inventory(&mut self, item_id: impl Into<ItemId>) -> &mut Self {
        let recipient = self.id();

        self.commands().queue(EquipFromInventory {
            recipient,
            item_id: item_id.into(),
        });

        self
    }
}
