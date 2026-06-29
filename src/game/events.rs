use crate::prelude::*;
use bevy::prelude::*;

#[derive(Event)]
pub struct PlayerContinued {
    pub from: AreaId,
}

#[derive(Event, Debug)]
pub struct PlayerChose {
    pub from: AreaId,
    pub to: AreaId,
    pub chosen_id: String,
}
