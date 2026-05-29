use bevy::prelude::*;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Description(pub String);

#[derive(Component)]
pub struct Health(pub usize);
