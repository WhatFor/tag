use crate::prelude::*;
use bevy::prelude::*;

use std::cmp::max;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_damage);
        app.add_observer(on_player_died);
        app.add_observer(on_enemy_died);
        app.add_observer(on_apply_effect);
    }
}

fn on_damage(trigger: On<Damage>, mut commands: Commands, mut healths: Query<&mut Health>) {
    let Ok(mut health) = healths.get_mut(trigger.damaged) else {
        return;
    };

    health.0 = max(0, health.0 - trigger.amount);

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
    mut healths: Query<&mut Health>,
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
            info!("Inflicting {:?} on target", status);
        }
        Effect::Heal { amount } => {
            info!("Healing target {} health", amount);
        }
        Effect::Buff { stats, duration } => {
            info!("Applying buff for {} of {:?}", duration, stats);
        }
        Effect::Cleanse { status } => {
            info!("Removing {:?} status via cleanse", status);
        }
    }
}
