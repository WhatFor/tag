use crate::prelude::*;
use bevy::prelude::*;

pub struct GiveItem {
    pub recipient: Entity,
    pub item_id: String,
    pub count: u32,
}

pub struct TakeItem {
    pub holder: Entity,
    pub item_id: String,
    pub count: u32,
}

pub trait GiveItemExt {
    fn give(&mut self, item_id: impl Into<String>, count: u32) -> &mut Self;
}

pub trait TakeItemExt {
    fn take(&mut self, item_id: impl Into<String>, count: u32) -> &mut Self;
}

impl Command for GiveItem {
    fn apply(self, world: &mut World) {
        // Find the item definition
        let Some(def) = world.resource::<ItemStore>().get(&self.item_id).cloned() else {
            warn!("Give item failed to find item. ID: {}", self.item_id);
            return;
        };

        let Some(mut inventory) = world.get_mut::<Inventory>(self.recipient) else {
            warn!("give: recipient has no Inventory");
            return;
        };

        if def.stackable {
            // Either bump the count
            if let Some(stack) = inventory.iter_mut().find(|i| *i.item_id == def.id) {
                stack.count += self.count;
                return;
            }

            // Or add a new entry for the count
            inventory.push(ItemStack {
                item_id: ItemId(def.id),
                count: self.count,
            });
        } else {
            // Push individual, unstackable entries
            for _ in 0..self.count {
                inventory.push(ItemStack {
                    item_id: ItemId(def.id.clone()),
                    count: 1,
                });
            }
        }
    }
}

impl Command for TakeItem {
    fn apply(self, world: &mut World) {
        // Find the item definition
        let Some(def) = world.resource::<ItemStore>().get(&self.item_id).cloned() else {
            warn!("Give item failed to find item. ID: {}", self.item_id);
            return;
        };

        let Some(mut inventory) = world.get_mut::<Inventory>(self.holder) else {
            warn!("give: recipient has no Inventory");
            return;
        };

        if def.stackable {
            if let Some(stack) = inventory.iter_mut().find(|i| *i.item_id == def.id) {
                stack.count -= self.count;
            }
        } else {
            if let Some((index, _)) = inventory
                .iter()
                .enumerate()
                .find(|(_, item)| *item.item_id == def.id)
            {
                inventory.remove(index);
            }
        }
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

impl TakeItemExt for EntityCommands<'_> {
    fn take(&mut self, item_id: impl Into<String>, count: u32) -> &mut Self {
        let holder = self.id();

        self.commands().queue(TakeItem {
            item_id: item_id.into(),
            holder,
            count,
        });

        self
    }
}
