use bevy::prelude::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
pub struct Health(pub usize);

// Base stats of an Entity
#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct Stats {
    pub strength: i32,
    pub agility: i32,
    pub intelligence: i32,
    pub speed: i32,
    pub armour: i32,
}

// Used when an item or effect modifies base stats
#[derive(Component, Clone)]
pub struct StatBonus(pub Stats);

// Used when an item or effect modifies armour
#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
pub struct ArmourBonus(i32);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Reflect)]
pub enum EquipmentSlot {
    Helm,
    Cloak,
    Chest,
    Legs,
    Boots,
    Ring,
    MainHand,
    OffHand,
}

#[derive(Component, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct Equippable(pub EquipmentSlot);

#[derive(Component, Default, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct Equipment(pub HashMap<EquipmentSlot, Entity>);

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
pub struct Gold(pub u32);

impl std::ops::Add for Stats {
    type Output = Stats;

    fn add(self, rhs: Stats) -> Stats {
        Stats {
            strength: self.strength + rhs.strength,
            agility: self.agility + rhs.agility,
            intelligence: self.intelligence + rhs.intelligence,
            speed: self.speed + rhs.speed,
            armour: self.armour + rhs.armour,
        }
    }
}
