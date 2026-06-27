use bevy::prelude::*;

#[derive(Event)]
pub struct SaveRequested;

#[derive(Event)]
pub struct SaveDeleted;

#[derive(Event)]
pub struct SettingsUpdated;

#[derive(Event)]
pub struct LoadCheckpoint {
    pub area_id: String,
}
