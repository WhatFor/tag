use crate::prelude::*;
use bevy::prelude::*;

use crate::game::combat::fsm::CombatPhasePlugin;
use rand::RngExt;
use std::cmp::{max, min};

mod fsm;

pub mod events;
pub mod move_plan;
pub mod resources;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CombatPhasePlugin);

        app.add_systems(
            Update,
            recompute_effective_player_stats.run_if(in_state(GameState::Playing)),
        );

        app.add_systems(
            Update,
            recompute_effective_enemy_stats.run_if(in_state(PlayState::InCombat)),
        );

        app.add_observer(on_heal);
        app.add_observer(on_damage);
        app.add_observer(on_player_died);
        app.add_observer(on_enemy_died);
        app.add_observer(on_apply_effect);
    }
}

fn recompute_effective_player_stats(
    mut player: Query<
        (&Stats, &Equipment, &Statuses, &mut EffectiveStats),
        (
            With<Player>,
            Or<(Changed<Stats>, Changed<Equipment>, Changed<Statuses>)>,
        ),
    >,
    //bonuses: Query<&StatBonus>,
) {
    for (base, _equipment, _statuses, mut out) in &mut player {
        // TODO: Real calc
        let next = *base;

        if out.0 != next {
            out.0 = next;
        }
    }
}

fn recompute_effective_enemy_stats(
    mut player: Query<
        (&Stats, &Statuses, &mut EffectiveStats),
        (With<Enemy>, Or<(Changed<Stats>, Changed<Statuses>)>),
    >,
    //bonuses: Query<&StatBonus>,
) {
    for (base, _statuses, mut out) in &mut player {
        // TODO: Real calc
        let next = *base;

        if out.0 != next {
            out.0 = next;
        }
    }
}

fn on_heal(trigger: On<Heal>, mut healths: Query<(&mut Health, &MaxHealth)>) {
    let Ok((mut health, max_health)) = healths.get_mut(trigger.healed) else {
        return;
    };

    health.0 = min(max_health.0, health.0 + trigger.amount);
}

fn on_damage(
    trigger: On<Damage>,
    mut commands: Commands,
    mut healths: Query<(&mut Health, &Statuses)>,
) {
    let Ok((mut health, statuses)) = healths.get_mut(trigger.damaged) else {
        return;
    };

    let resistance_amount: i32 = statuses
        .0
        .iter()
        .filter_map(|f| match f.effect {
            StatusEffect::DamageResistance {
                damage_type,
                potency,
            } if damage_type == trigger.damage_type => Some(potency),
            _ => None,
        })
        .sum();

    let reduced_damage = max(0, trigger.amount - resistance_amount);

    health.0 = max(0, health.0 - reduced_damage);

    if health.0 == 0 {
        commands.trigger(Died {
            died: trigger.damaged,
        });
    }
}

fn on_player_died(trigger: On<Died>, players: Query<(), With<Player>>, mut commands: Commands) {
    if players.get(trigger.died).is_err() {
        return;
    }

    commands.trigger(PlayerDied {
        reason: DeathReason::NoHealth,
    });
}

fn on_enemy_died(trigger: On<Died>, enemies: Query<(), With<Enemy>>, mut commands: Commands) {
    if enemies.get(trigger.died).is_err() {
        return;
    }

    commands.entity(trigger.died).despawn();
}

fn on_apply_effect(
    trigger: On<ApplyEffect>,
    mut statuses: Query<&mut Statuses>,
    mut rng: ResMut<GameRng>,
    mut commands: Commands,
) {
    let target = trigger.target;

    match &trigger.effect {
        Effect::Inflict {
            status,
            potency,
            duration,
            chance,
        } => {
            let Ok(mut target_statuses) = statuses.get_mut(target) else {
                return;
            };

            if !rng.0.random_bool((*chance as f64).clamp(0.0, 1.0)) {
                return;
            }

            target_statuses.0.push(ActiveStatus {
                effect: StatusEffect::DamageOverTime {
                    kind: *status,
                    potency: *potency,
                },
                turns: *duration,
            });
        }
        Effect::Heal { amount } => {
            commands.trigger(Heal {
                amount: *amount,
                healed: target,
            });
        }
        Effect::Buff { stats, duration } => {
            let Ok(mut target_statuses) = statuses.get_mut(target) else {
                return;
            };

            target_statuses.0.push(ActiveStatus {
                effect: StatusEffect::StatModifier { stats: *stats },
                turns: *duration,
            });
        }
        Effect::Resistance {
            damage_type,
            potency,
            duration,
        } => {
            let Ok(mut target_statuses) = statuses.get_mut(target) else {
                return;
            };

            target_statuses.0.push(ActiveStatus {
                effect: StatusEffect::DamageResistance {
                    damage_type: *damage_type,
                    potency: *potency,
                },
                turns: *duration,
            });
        }
        Effect::Cleanse { status } => {
            let Ok(mut target_statuses) = statuses.get_mut(target) else {
                return;
            };

            target_statuses.0.retain(|e| {
                !matches!(
                    e.effect,
                    StatusEffect::DamageOverTime { kind, .. } if kind == *status
                )
            });
        }
    }
}
