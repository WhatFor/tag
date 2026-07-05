use bevy::prelude::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
pub struct Health(pub usize);

#[derive(Debug, Clone, Deserialize)]
pub enum Effect {
    Inflict {
        status: StatusKind,
        potency: i32,
        duration: u32,
        chance: f32,
    },
    Heal {
        amount: i32,
    },
    Buff {
        stats: Stats,
        duration: u32,
    },
    Cleanse {
        status: StatusKind,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum StatusKind {
    Burn,
    Bleed,
    Poison,
}

pub struct ActiveStatus {
    pub kind: StatusKind,
    pub potency: i32,
    pub turns: u32,
}

#[derive(Component, Default)]
pub struct Statuses(pub Vec<ActiveStatus>);

// Base stats of an Entity
#[derive(Component, Debug, Reflect, Default, Clone, Copy, Serialize, Deserialize)]
#[reflect(Component)]
#[serde(default)]
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

impl std::fmt::Display for EquipmentSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            EquipmentSlot::Helm => "Helm",
            EquipmentSlot::Cloak => "Cloak",
            EquipmentSlot::Chest => "Chest",
            EquipmentSlot::Legs => "Legs",
            EquipmentSlot::Boots => "Boots",
            EquipmentSlot::Ring => "Ring",
            EquipmentSlot::MainHand => "Main hand",
            EquipmentSlot::OffHand => "Off hand",
        };

        f.write_str(label)
    }
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

impl Stats {
    pub fn non_zero_stats(&self) -> Vec<(&'static str, String, &'static str)> {
        let mut vec = Vec::new();

        if self.strength != 0 {
            vec.push(("Strength", stat_to_string(self.strength), "strength"));
        }

        if self.agility != 0 {
            vec.push(("Agility", stat_to_string(self.agility), "agility"));
        }

        if self.intelligence != 0 {
            vec.push((
                "Intelligence",
                stat_to_string(self.intelligence),
                "intelligence",
            ));
        }

        if self.speed != 0 {
            vec.push(("Speed", stat_to_string(self.speed), "speed"));
        }

        if self.armour != 0 {
            vec.push(("Armour", stat_to_string(self.armour), "armour"));
        }

        vec
    }
}

fn stat_to_string(stat: i32) -> String {
    if stat == 0 {
        String::from("0")
    } else {
        format!("{stat:+}")
    }
}
