use bevy::prelude::*;

use crate::{
    assets::item_loader::{ItemDef, ItemStore},
    world::{
        bundles::{item, item_stack},
        components::{Inventory, ItemId, ItemStack},
    },
};

pub struct GiveItem {
    pub recipient: Entity,
    pub item_id: String,
    pub count: u32,
}

pub trait GiveItemExt {
    fn give(&mut self, item_id: impl Into<String>, count: u32) -> &mut Self;
}

impl Command for GiveItem {
    fn apply(self, world: &mut World) {
        // Find the item definition
        let def = match world.resource::<ItemStore>().0.get(&self.item_id) {
            Some(def) => def.clone(),
            None => {
                warn!("Give item failed to find item. ID: {}", self.item_id);
                return;
            }
        };

        if def.stackable {
            if let Some(stack_entity) = find_item_stack(world, self.recipient, &def) {
                if let Some(mut stack) = world.get_mut::<ItemStack>(stack_entity) {
                    stack.0 += self.count;
                    return;
                }
            };
            // find inventory for target recipient, check if they already have items, and merge
        }

        // Spawn the items
        let item_entity = if def.stackable {
            world.spawn(item_stack(&def, self.count)).id()
        } else {
            world.spawn(item(&def)).id()
        };

        // Find the recipient
        let mut recipient = world.entity_mut(self.recipient);

        // Either add the item(s) to an existing inventory, or create one and add
        match recipient.get_mut::<Inventory>() {
            Some(mut invent) => {
                invent.0.push(item_entity);
            }
            None => {
                recipient.insert(Inventory(vec![item_entity]));
            }
        }
    }
}

fn find_item_stack(world: &World, recipient: Entity, item: &ItemDef) -> Option<Entity> {
    // Find the item definition
    let def = match world.resource::<ItemStore>().0.get(&item.id) {
        Some(def) => def,
        None => {
            warn!("Find item failed to find item. ID: {}", item.id);
            return None;
        }
    };

    // Check that the given Item is in the inventory and that it's stackable. If so, return it.
    match world.get::<Inventory>(recipient) {
        Some(invent) => invent.0.iter().copied().find(|&item| {
            let is_match = world.get::<ItemId>(item).is_some_and(|id| id.0 == def.id);
            let stackable = world.get::<ItemStack>(item).is_some();
            is_match && stackable
        }),
        None => None,
    }
}

impl GiveItemExt for EntityCommands<'_> {
    fn give(&mut self, item_id: impl Into<String>, count: u32) -> &mut Self {
        let recipient = self.id();

        self.commands().queue(GiveItem {
            item_id: item_id.into(),
            recipient,
            count,
        });

        self
    }
}
