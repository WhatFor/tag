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

#[derive(Resource, Default)]
pub struct CombatLog {
    pub lines: Vec<CombatLogLine>,
}

pub enum CombatLogLine {
    Text(String),
}

#[derive(Resource, Default)]
pub struct TurnOrder {
    pub queue: Vec<Entity>,
    pub cursor: usize,
}

#[derive(Resource)]
pub struct TurnTimer(pub Timer);

impl Default for TurnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.6, TimerMode::Once))
    }
}
