use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct CurrentArea(pub Entity);

#[derive(Component)]
pub struct Inventory(pub Vec<Entity>);
