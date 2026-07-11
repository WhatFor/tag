use bevy::prelude::*;

#[derive(Event)]
pub enum PlayerCombatAction {
    Attack(Entity), // todo: target
    Defend,
    // todo: specials, other stuff(?)
}
