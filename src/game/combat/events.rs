use bevy::prelude::*;

#[derive(Event)]
pub enum PlayerCombatAction {
    Attack, // todo: target
    Defend,
    // todo: specials, other stuff(?)
}
