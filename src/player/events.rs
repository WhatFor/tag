use bevy::prelude::*;

#[derive(Event)]
pub struct PlayerDied {
    pub reason: DeathReason,
}

#[derive(Clone)]
pub enum DeathReason {
    NoHealth,
    Ending(String),
}

#[derive(Resource)]
pub struct LastDeath(pub DeathReason);
