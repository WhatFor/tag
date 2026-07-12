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

        app.add_observer(on_combatant_turn_start);
        app.add_observer(on_heal);
        app.add_observer(on_damage);
        app.add_observer(on_enemy_died);
        app.add_observer(on_apply_effect);
    }
}

pub fn in_combat_phase(phase: CombatPhase) -> impl Fn(Option<Res<CombatState>>) -> bool {
    move |state: Option<Res<CombatState>>| state.is_some_and(|s| s.phase == phase)
}

fn recompute_effective_player_stats(
    mut player: Query<
        (&Stats, &Equipment, &Statuses, &mut EffectiveStats),
        (
            With<Player>,
            Or<(Changed<Stats>, Changed<Equipment>, Changed<Statuses>)>,
        ),
    >,
    bonuses: Query<&StatBonus>,
) {
    // StatBonus comes from items
    // Statuses may include a Stat bonus, from stuff like potions/defending/etc.
    for (base, equipment, statuses, mut out) in &mut player {
        let stats_from_equipment: Vec<&StatBonus> = equipment
            .0
            .values()
            .filter_map(|equipment_entity| bonuses.get(*equipment_entity).ok())
            .collect();

        let next = effective_stats(base, stats_from_equipment, statuses);

        if out.0 != next.0 {
            out.0 = next.0;
        }
    }
}

fn recompute_effective_enemy_stats(
    mut enemies: Query<
        (&Stats, &Statuses, &mut EffectiveStats),
        (With<Enemy>, Or<(Changed<Stats>, Changed<Statuses>)>),
    >,
) {
    // Does not include equipment for enemies
    for (base, statuses, mut out) in &mut enemies {
        let next = effective_stats(base, vec![], statuses);

        if out.0 != next.0 {
            out.0 = next.0;
        }
    }
}

pub fn effective_stats(
    base: &Stats,
    equipment_bonuses: Vec<&StatBonus>,
    statuses: &Statuses,
) -> EffectiveStats {
    let stats_from_equipment: Stats = equipment_bonuses.iter().copied().map(|b| b.0).sum();

    let stats_from_statuses: Stats = statuses
        .0
        .iter()
        .filter_map(|status| match status.effect {
            StatusEffect::StatModifier { stats } => Some(stats),
            _ => None,
        })
        .sum();

    let total = *base + stats_from_equipment + stats_from_statuses;

    EffectiveStats(total)
}

fn on_combatant_turn_start(
    trigger: On<CombatantTurnStarted>,
    mut commands: Commands,
    mut params: Query<(&mut Statuses, Option<&Dead>)>,
) {
    // The given entity has just started a turn.
    //  - Tick DoTs
    //  - Tick all durations, removing if expired
    //
    let Ok((mut statuses, dead)) = params.get_mut(trigger.combatant) else {
        return;
    };

    // If dead, skip
    if dead.is_some() {
        return;
    }

    for status in statuses.0.iter_mut() {
        match status.effect {
            StatusEffect::DamageOverTime { kind, potency } => {
                commands.trigger(Damage {
                    damaged: trigger.combatant,
                    amount: potency,
                    damage_type: kind.get_damage_type(),
                });
            }
            _ => { /* No action needed */ }
        }

        status.turns -= 1;
    }

    statuses.0.retain(|s| s.turns > 0);
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
    mut params: Query<(&mut Health, &Statuses, &EffectiveStats)>,
) {
    let Ok((mut health, statuses, stats)) = params.get_mut(trigger.damaged) else {
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

    let reduced_damage = max(0, trigger.amount - resistance_amount - stats.0.armour);

    health.0 = max(0, health.0 - reduced_damage);

    if health.0 == 0 {
        commands.trigger(Died {
            died: trigger.damaged,
        });
    }
}

fn on_enemy_died(trigger: On<Died>, enemies: Query<(), With<Enemy>>, mut commands: Commands) {
    if enemies.get(trigger.died).is_err() {
        return;
    }

    commands.entity(trigger.died).insert(Dead);
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
