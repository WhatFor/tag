use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct AwaitingPlayerAction(pub bool);

#[derive(Default, Debug, Eq, PartialEq)]
pub enum CombatPhase {
    #[default]
    StartOfCombat,
    StartOfRound,
    RoundCombat,
    EndOfRound,
    EndOfCombat,
}

#[derive(Resource, Default)]
pub struct CombatState {
    pub phase: CombatPhase,
}
