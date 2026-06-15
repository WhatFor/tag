use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub version: u32,
    pub current_area_id: String,
    pub health: usize,
    pub inventory: Vec<SavedItem>,
}

#[derive(Serialize, Deserialize)]
pub struct SavedItem {
    pub item_id: String,
    pub count: u32,
}
