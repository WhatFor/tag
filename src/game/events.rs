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

#[derive(EntityEvent)]
pub struct Damage {
    #[event_target]
    pub damaged: Entity,
    pub amount: usize,
}

#[derive(EntityEvent)]
pub struct Died {
    #[event_target]
    pub died: Entity,
}

#[derive(EntityEvent)]
pub struct GiveGold {
    #[event_target]
    pub beneficiary: Entity,
    pub amount: u32,
}

#[derive(EntityEvent)]
pub struct ApplyEffect {
    #[event_target]
    pub target: Entity,
    pub effect: Effect,
}
