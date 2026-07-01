use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
pub struct Health(pub usize);

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
pub struct Gold(pub u32);
