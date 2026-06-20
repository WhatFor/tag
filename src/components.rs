use bevy::prelude::*;

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
pub struct DisplayName(pub String);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Description(pub String);

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
pub struct Health(pub usize);
