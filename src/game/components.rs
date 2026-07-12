use bevy::prelude::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Dead;

#[derive(Component)]
pub struct CombatSlot(pub usize);

#[derive(Component)]
pub struct EnemyId(pub String);

#[derive(Debug, Clone, Deserialize)]
pub struct EnemyLoot {
    pub item_id: String,
    pub quantity: u32,
    pub chance: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub enum EnemyMove {
    BasicAttack {
        name: String,
        potency: i32,
        damage_type: DamageType,
    },
    SpecialAttack {
        name: String,
        potency: i32,
        damage_type: DamageType,
        cooldown: i32,
        start_on_cooldown: bool,
        telegraph: Option<String>,
    },
    Defend {
        potency: i32,
        cooldown: i32,
    },
}

#[derive(Component, Default)]
pub struct MoveSet(pub Vec<EnemyMove>);

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
pub struct Health(pub i32);

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
pub struct MaxHealth(pub i32);

#[derive(Debug, Clone, Deserialize)]
pub enum Effect {
    Inflict {
        status: DotKind,
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
    Resistance {
        damage_type: DamageType,
        potency: i32,
        duration: u32,
    },
    Cleanse {
        status: DotKind,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum DotKind {
    Burn,
    Bleed,
    Poison,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum DamageKind {
    None,
    Physical,
    Magical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum AttackType {
    Basic,
    Special,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum DamageType {
    None,
    Slash,
    Stab,
    Smash,
    Fire,
    Ice,
    Poison,
    Necrotic,
    Lightning,
}

impl std::fmt::Display for DamageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            DamageType::None => "Typeless",
            DamageType::Slash => "Slashing",
            DamageType::Stab => "Stabbing",
            DamageType::Smash => "Smashing",
            DamageType::Fire => "Fire",
            DamageType::Ice => "Ice",
            DamageType::Poison => "Poison",
            DamageType::Necrotic => "Necrotic",
            DamageType::Lightning => "Lightning",
        };

        f.write_str(label)
    }
}

pub enum StatusEffect {
    DamageOverTime {
        kind: DotKind,
        potency: i32,
    },
    StatModifier {
        stats: Stats,
    },
    DamageResistance {
        damage_type: DamageType,
        potency: i32,
    },
}

pub struct ActiveStatus {
    pub effect: StatusEffect,
    pub turns: u32,
}

#[derive(Component, Default)]
pub struct Statuses(pub Vec<ActiveStatus>);

// Base stats of an Entity
#[derive(
    Component, Debug, Reflect, Default, Clone, Copy, Serialize, Deserialize, Eq, PartialEq,
)]
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

// A 'cached' result of the effective stats (base + equipment + buffs)
#[derive(Component, Clone, Copy, Default)]
pub struct EffectiveStats(pub Stats);

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

impl std::iter::Sum for Stats {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Stats::default(), |acc, stats| acc + stats)
    }
}

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

impl DotKind {
    pub fn get_damage_type(&self) -> DamageType {
        match self {
            DotKind::Burn => DamageType::Fire,
            DotKind::Bleed => DamageType::None,
            DotKind::Poison => DamageType::Poison,
        }
    }
}

impl DamageType {
    pub fn primary_stat(&self, stats: &Stats) -> i32 {
        match self {
            DamageType::None => 0,
            DamageType::Slash => stats.strength,
            DamageType::Stab => stats.strength,
            DamageType::Smash => stats.strength,
            DamageType::Fire => stats.intelligence,
            DamageType::Ice => stats.intelligence,
            DamageType::Poison => stats.intelligence,
            DamageType::Necrotic => stats.intelligence,
            DamageType::Lightning => stats.intelligence,
        }
    }

    pub fn get_damage_kind(&self) -> DamageKind {
        match self {
            DamageType::None => DamageKind::None,
            DamageType::Slash => DamageKind::Physical,
            DamageType::Stab => DamageKind::Physical,
            DamageType::Smash => DamageKind::Physical,
            DamageType::Fire => DamageKind::Magical,
            DamageType::Ice => DamageKind::Magical,
            DamageType::Poison => DamageKind::Magical,
            DamageType::Necrotic => DamageKind::Magical,
            DamageType::Lightning => DamageKind::Magical,
        }
    }
}
