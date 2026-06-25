use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct CurrentArea(pub Entity);

#[derive(Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct LastCheckpointArea(pub Entity);

impl CurrentArea {
    pub fn entity(&self) -> Entity {
        self.0
    }
}
