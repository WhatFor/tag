use crate::prelude::*;
use bevy::prelude::*;

#[derive(Event)]
pub struct PlayerContinued;

#[derive(Event)]
pub struct PlayerChose(pub AreaId);
