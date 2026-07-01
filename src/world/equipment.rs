use crate::prelude::*;
use bevy::prelude::*;

pub struct EquipItem {
    pub recipient: Entity,
    pub item_id: String,
}

pub struct EquipFromInventory {
    pub recipient: Entity,
    pub item: Entity,
}

pub trait EquipItemExt {
    fn spawn_and_equip(&mut self, item_id: impl Into<String>) -> &mut Self;

    fn equip_from_inventory(&mut self, item: Entity) -> &mut Self;
}

impl Command for EquipItem {
    fn apply(self, world: &mut World) {
        let Some(def) = world.resource::<ItemStore>().get(&self.item_id).cloned() else {
            warn!("Equip failed to find item. ID: {}", self.item_id);
            return;
        };

        let Some(slot) = def.slot else {
            warn!("Item is not equippable. ID: {}", self.item_id);
            return;
        };

        let item_entity = world.spawn((item(&def), Equippable(slot))).id();

        let mut recipient = world.entity_mut(self.recipient);

        match recipient.get_mut::<Equipment>() {
            Some(mut equipment) => {
                if let Some(_previous) = equipment.insert(slot, item_entity) {
                    let _ = world.try_despawn(_previous);
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
        let Some(slot) = world.entity(self.item).get::<Equippable>().map(|e| e.0) else {
            warn!(
                "equip_from_inventory: item {:?} is not equippable",
                self.item
            );
            return;
        };

        let Some(mut inventory) = world.get_mut::<Inventory>(self.recipient) else {
            warn!("equip_from_inventory: recipient has no Inventory");
            return;
        };

        let Some(pos) = inventory.iter().position(|&e| e == self.item) else {
            warn!(
                "equip_from_inventory: item {:?} not in inventory",
                self.item
            );
            return;
        };

        inventory.remove(pos);

        let displaced = {
            let Some(mut equipment) = world.get_mut::<Equipment>(self.recipient) else {
                warn!("equip_from_inventory: recipient has no Equipment");
                return;
            };

            equipment.insert(slot, self.item)
        };

        if let Some(previous) = displaced {
            if let Some(mut inventory) = world.get_mut::<Inventory>(self.recipient) {
                inventory.push(previous);
            }
        }
    }
}

impl EquipItemExt for EntityCommands<'_> {
    fn spawn_and_equip(&mut self, item_id: impl Into<String>) -> &mut Self {
        let recipient = self.id();

        self.commands().queue(EquipItem {
            item_id: item_id.into(),
            recipient,
        });

        self
    }

    fn equip_from_inventory(&mut self, item: Entity) -> &mut Self {
        let recipient = self.id();

        self.commands()
            .queue(EquipFromInventory { recipient, item });

        self
    }
}
