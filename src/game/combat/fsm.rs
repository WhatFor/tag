use crate::prelude::*;
use bevy::prelude::*;

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
    phase: CombatPhase,
}

#[derive(Resource, Default)]
pub struct TurnOrder {
    queue: Vec<Entity>,
    cursor: usize,
}

#[derive(Resource, Default)]
pub struct CombatLog {
    lines: Vec<CombatLogLine>,
}

pub enum CombatLogLine {
    Text(String),
}

#[derive(Resource, Default)]
pub struct AwaitingPlayerAction(pub bool);

#[derive(Event)]
enum PlayerAction {
    Attack, // todo: target
    Defend,
    // todo: specials, other stuff(?)
}

pub struct CombatPhasePlugin;

impl Plugin for CombatPhasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayState::InCombat), init);

        app.add_systems(
            Update,
            (
                start_combat.run_if(in_phase(CombatPhase::StartOfCombat)),
                start_round.run_if(in_phase(CombatPhase::StartOfRound)),
                round_combat.run_if(in_phase(CombatPhase::RoundCombat)),
                end_round.run_if(in_phase(CombatPhase::EndOfRound)),
                end_combat.run_if(in_phase(CombatPhase::EndOfCombat)),
            )
                .run_if(in_state(PlayState::InCombat)),
        );

        app.add_observer(on_player_action);

        app.add_systems(OnExit(PlayState::InCombat), destroy);
    }
}

fn in_phase(phase: CombatPhase) -> impl Fn(Option<Res<CombatState>>) -> bool {
    move |state: Option<Res<CombatState>>| state.is_some_and(|s| s.phase == phase)
}

fn init(mut commands: Commands) {
    commands.init_resource::<TurnOrder>();
    commands.init_resource::<CombatLog>();
    commands.init_resource::<AwaitingPlayerAction>();
    commands.init_resource::<CombatState>();
}

fn start_combat(
    mut commands: Commands,
    mut state: ResMut<CombatState>,
    mut log: ResMut<CombatLog>,
    area: Single<&CurrentArea, With<Player>>,
    all_area_content: Query<&AreaContent, With<Area>>,
    enemy_store: Res<EnemyStore>,
) {
    info!("[Combat] Entered StartCombat");

    let Ok(AreaContent::Combat {
        start_lines,
        enemy_ids,
        ..
    }) = all_area_content.get(area.0)
    else {
        return;
    };

    for line in start_lines {
        log.lines.push(CombatLogLine::Text(line.clone()));
    }

    for id in enemy_ids {
        let Some(enemy) = enemy_store.0.get(id) else {
            continue;
        };

        commands.spawn((
            Enemy,
            Health(enemy.max_health),
            MaxHealth(enemy.max_health),
            DisplayName(enemy.display_name.clone()),
            Statuses::default(),
            enemy.stats,
            DespawnOnExit(PlayState::InCombat),
            // TODO: Do I want to spawn Attacks here?
        ));
    }

    state.phase = CombatPhase::StartOfRound;
}

fn start_round(
    player: Single<(Entity, &Stats), With<Player>>,
    enemies: Query<(Entity, &Stats), With<Enemy>>,
    mut turn_order: ResMut<TurnOrder>,
    mut state: ResMut<CombatState>,
) {
    info!("[Combat] Entered StartRound");

    // TODO: This needs to account for buffs, not just raw Stats

    turn_order.queue = {
        let player = (player.0, *player.1);
        let mut combatants = vec![player];

        combatants.extend(enemies.iter().map(|(e, s)| (e, *s)));

        // Fastest first; Tie-break using agility.
        combatants.sort_by(|a, b| {
            b.1.speed
                .cmp(&a.1.speed)
                .then(b.1.agility.cmp(&a.1.agility))
        });

        combatants.into_iter().map(|(e, _)| e).collect()
    };

    info!("[Combat] Turn order: {:?}", turn_order.queue);

    turn_order.cursor = 0;

    state.phase = CombatPhase::RoundCombat;
}

fn round_combat(
    mut commands: Commands,
    mut turn_order: ResMut<TurnOrder>,
    mut log: ResMut<CombatLog>,
    player: Single<Entity, With<Player>>,
    enemies: Query<(), With<Enemy>>,
    mut state: ResMut<CombatState>,
    mut awaiting_player: ResMut<AwaitingPlayerAction>,
) {
    info!("[Combat] Entered RoundCombat");

    let Some(&active_combatant) = turn_order.queue.get(turn_order.cursor) else {
        info!("[Combat] At end of Turn order queue. Moving to end of round...");
        state.phase = CombatPhase::EndOfRound;

        return;
    };

    // TODO: Tick dots

    if active_combatant == player.entity() {
        if awaiting_player.0 == false {
            info!("[Combat] Player turn...");
            awaiting_player.0 = true;
        }
    } else if let Ok(enemy) = enemies.get(active_combatant) {
        info!("[Combat] Enemy {:?} turn...", enemy);

        // TODO: Attack details hardcoded
        commands.trigger(Damage {
            damaged: *player,
            amount: 1,
            damage_type: DamageType::Slash,
        });

        log.lines.push(CombatLogLine::Text(format!(
            "The enemy hit you for 1 slashing damage."
        )));

        turn_order.cursor += 1;
    } else {
        info!("[Combat] Entity no longer exists. Skipping...");
        turn_order.cursor += 1;
    }
}

fn end_round(enemies: Query<(), With<Enemy>>, mut state: ResMut<CombatState>) {
    info!("[Combat] Entered EndRound");

    // TODO: Handle player death

    if enemies.is_empty() {
        state.phase = CombatPhase::EndOfCombat;
    } else {
        state.phase = CombatPhase::StartOfRound;
    }
}

fn end_combat(
    mut commands: Commands,
    mut play_state: ResMut<NextState<PlayState>>,
    mut log: ResMut<CombatLog>,
    area: Single<(&AreaId, &CurrentArea), With<Player>>,
    all_area_content: Query<&AreaContent, With<Area>>,
) {
    info!("[Combat] Entered EndCombat");

    let Ok(AreaContent::Combat {
        win_lines,
        lose_lines,
        ..
    }) = all_area_content.get(area.1.0)
    else {
        return;
    };

    // TODO: pushing 'win_lines", but need to check if player lost or not
    for line in win_lines {
        log.lines.push(CombatLogLine::Text(line.clone()));
    }

    // TODO: looting

    play_state.set(PlayState::Exploring);

    commands.trigger(PlayerContinued {
        from: area.0.clone(),
    });
}

fn on_player_action(
    trigger: On<PlayerAction>,
    mut commands: Commands,
    mut log: ResMut<CombatLog>,
    mut awaiting_player: ResMut<AwaitingPlayerAction>,
    mut turn_order: ResMut<TurnOrder>,
) {
    if awaiting_player.0 == false {
        return;
    }

    match *trigger {
        PlayerAction::Attack => {
            // TODO: who you attackin?
            //
            log.lines
                .push(CombatLogLine::Text(format!("You atack something.")));
        }
        PlayerAction::Defend => {
            log.lines
                .push(CombatLogLine::Text(format!("You defend, or something.")));
        }
    }

    // Move on to next turn in combat
    awaiting_player.0 = false;
    turn_order.cursor += 1;
}

fn destroy(mut commands: Commands) {
    commands.remove_resource::<TurnOrder>();
    commands.remove_resource::<CombatLog>();
    commands.remove_resource::<AwaitingPlayerAction>();
    commands.remove_resource::<CombatState>();
}
