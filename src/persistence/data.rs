use crate::prelude::*;
use bevy::prelude::*;

use crate::player::components::TakenPath;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CheckpointData {
    pub checkpoint_area_id: String,
    pub save_at_checkpoint: SaveData,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SaveData {
    pub version: u32,
    pub hardcore: bool,
    pub current_area_id: String,
    pub last_checkpoint_area_id: String,
    pub health: i32,
    pub max_health: i32,
    pub strength: i32,
    pub agility: i32,
    pub intelligence: i32,
    pub speed: i32,
    pub armour: i32,
    pub gold: u32,
    pub inventory: Vec<SavedItem>,
    pub equipped: Vec<EquippedItem>,
    pub path_taken: Vec<TakenPath>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SavedItem {
    pub item_id: String,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EquippedItem {
    pub item_id: String,
    pub slot: EquipmentSlot,
}

#[derive(Serialize, Deserialize)]
pub struct SettingsData {
    pub version: u32,
    pub audio: AudioSettings,
}

#[derive(Resource, Clone, Copy, Serialize, Deserialize)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub ambience_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.,
            ambience_volume: 1.,
            music_volume: 1.,
            sfx_volume: 1.,
        }
    }
}
