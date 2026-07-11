use crate::prelude::*;
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
    LeavingCombat,
}

#[derive(Default, Debug, Eq, PartialEq)]
pub enum CombatResult {
    #[default]
    Active,
    PlayerWon,
    PlayerLost,
}

#[derive(Resource, Default)]
pub struct CombatState {
    pub phase: CombatPhase,
    pub result: CombatResult,
}

#[derive(Resource, Default)]
pub struct CombatLog {
    pub lines: Vec<CombatLogLine>,
}

pub enum CombatLogLine {
    Text(String),
    Attack(CombatLogAttack),
    Defend(CombatLogDefend),
    Effect(CombatLogEffect),
    CombatResult(CombatLogResult),
}

pub struct CombatLogAttack {
    pub from: Entity,
    pub to: Entity,
    pub attack_name: String,
    pub attack_type: AttackType,
    pub attack_damage: i32,
    pub damage_type: DamageType,
}

pub struct CombatLogDefend {
    pub entity: Entity,
    pub potency: i32,
}

pub struct CombatLogEffect {
    pub entity: Entity,
    pub effect: Effect,
}

pub struct CombatLogResult {
    pub message: String,
    pub player_won: bool,
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
