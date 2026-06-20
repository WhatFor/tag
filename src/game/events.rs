use crate::prelude::*;
use bevy::prelude::*;

#[derive(Event)]
pub struct PlayerContinued;

#[derive(Event, Deref, Debug)]
pub struct PlayerChose(pub AreaId);

#[derive(Event)]
pub struct PlayerGameOver;
