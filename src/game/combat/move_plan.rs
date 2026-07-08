use crate::prelude::*;
use bevy::prelude::*;

use rand::RngExt;

const QUEUE_LEN: usize = 3;
const SPECIAL_CHANCE: f64 = 0.5;

#[derive(Component)]
pub struct MovePlan {
    pub queue: std::collections::VecDeque<usize>,
    cooldowns: Vec<i32>,
}

impl MovePlan {
    pub fn new(moves: &[EnemyMove], rng: &mut impl rand::Rng) -> Self {
        let mut cooldowns: Vec<i32> = moves
            .iter()
            .map(|m| match m {
                EnemyMove::SpecialAttack {
                    cooldown,
                    start_on_cooldown: true,
                    ..
                } => *cooldown,
                _ => 0,
            })
            .collect();

        let mut queue = std::collections::VecDeque::with_capacity(QUEUE_LEN);

        for _ in 0..QUEUE_LEN {
            queue.push_back(plan_next_move(moves, &mut cooldowns, rng));
        }

        Self { queue, cooldowns }
    }

    pub fn advance(&mut self, moves: &[EnemyMove], rng: &mut impl rand::Rng) {
        self.queue.pop_front();
        self.queue
            .push_back(plan_next_move(moves, &mut self.cooldowns, rng));
    }
}

fn plan_next_move(moves: &[EnemyMove], cooldowns: &mut [i32], rng: &mut impl rand::Rng) -> usize {
    for cd in cooldowns.iter_mut() {
        *cd = (*cd - 1).max(0);
    }

    let mut specials = Vec::new();
    let mut normals = Vec::new();

    for (i, m) in moves.iter().enumerate() {
        match m {
            EnemyMove::BasicAttack { .. } => normals.push(i),
            EnemyMove::Defend { .. } if cooldowns[i] == 0 => normals.push(i),
            EnemyMove::SpecialAttack { .. } if cooldowns[i] == 0 => specials.push(i),
            _ => {}
        }
    }

    let chosen = if !specials.is_empty() && rng.random_bool(SPECIAL_CHANCE) {
        specials[rng.random_range(0..specials.len())]
    } else {
        normals[rng.random_range(0..normals.len())]
    };

    match &moves[chosen] {
        EnemyMove::SpecialAttack { cooldown, .. } | EnemyMove::Defend { cooldown, .. } => {
            cooldowns[chosen] = *cooldown;
        }
        EnemyMove::BasicAttack { .. } => {}
    }

    chosen
}
