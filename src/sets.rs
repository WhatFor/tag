use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayingSet {
    InitialiseUI,
    SpawnAreas,
    SpawnPlayer,
    SpawnCharacters,
    SpawnItems,
}
