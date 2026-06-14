use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct CurrentArea(pub Entity);
