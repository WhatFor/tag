use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SaveData {
    version: u32,
    current_area_id: String,
    health: usize,
    inventory: Vec<SavedItem>,
}

#[derive(Serialize, Deserialize)]
struct SavedItem {
    item_id: String,
    count: u32,
}
