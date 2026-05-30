use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayingSet {
    SpawnWorld,
    SpawnPlayer,
    SpawnCharacters,
}
