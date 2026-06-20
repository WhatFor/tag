use bevy::prelude::*;

#[derive(Event, Deref)]
pub struct PlayerEnteredArea(pub Entity);
