use bevy::prelude::*;

#[derive(Component)]
pub struct DisplayName(pub String);

#[derive(Component)]
#[allow(dead_code)]
pub struct Description(pub String);

#[derive(Component)]
#[allow(dead_code)]
pub struct Health(pub usize);
