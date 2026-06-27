use bevy::prelude::*;

use crate::components::TakenPath;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CheckpointData {
    pub checkpoint_area_id: String,
    pub save_at_checkpoint: SaveData,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SaveData {
    pub version: u32,
    pub current_area_id: String,
    pub last_checkpoint_area_id: String,
    pub health: usize,
    pub inventory: Vec<SavedItem>,
    pub path_taken: Vec<TakenPath>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SavedItem {
    pub item_id: String,
    pub count: u32,
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
