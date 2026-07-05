use crate::prelude::*;
use bevy::prelude::*;

#[derive(Debug, Eq, PartialEq)]
pub enum CombatPhase {
    StartOfCombat,
    StartOfRound,
    RoundCombat,
    EndOfRound,
    EndOfCombat,
}

#[derive(Resource)]
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

        app.add_systems(OnExit(PlayState::InCombat), destroy);
    }
}

fn in_phase(phase: CombatPhase) -> impl Fn(Option<Res<CombatState>>) -> bool {
    move |state: Option<Res<CombatState>>| state.is_some_and(|s| s.phase == phase)
}

fn init(mut commands: Commands) {
    commands.init_resource::<TurnOrder>();
    commands.init_resource::<CombatLog>();
    commands.insert_resource(CombatState {
        phase: CombatPhase::StartOfCombat,
    });
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

fn start_round(mut commands: Commands, mut state: ResMut<CombatState>) {
    info!("[Combat] Entered StartRound");
    // Re-calc turn order based on speed. On tie, look at agility. On tie, choose player.
    // Reset the turn order cursor.
    state.phase = CombatPhase::RoundCombat;
}

fn round_combat(mut commands: Commands, mut state: ResMut<CombatState>) {
    info!("[Combat] Entered RoundCombat");
    // Do combat! loop over turn-order queue based on cursor...
    state.phase = CombatPhase::EndOfRound;
}

fn end_round(mut commands: Commands, mut state: ResMut<CombatState>) {
    info!("[Combat] Entered EndRound");
    // TODO: Check if player/enemies are dead
    state.phase = CombatPhase::EndOfCombat;
}

fn end_combat(
    mut commands: Commands,
    mut play_state: ResMut<NextState<PlayState>>,
    mut log: ResMut<CombatLog>,
    area: Single<&CurrentArea, With<Player>>,
    all_area_content: Query<&AreaContent, With<Area>>,
) {
    info!("[Combat] Entered EndCombat");

    let Ok(AreaContent::Combat {
        win_lines,
        lose_lines,
        ..
    }) = all_area_content.get(area.0)
    else {
        return;
    };

    // TODO: pushing 'win_lines", but need to check if player lost or not
    for line in win_lines {
        log.lines.push(CombatLogLine::Text(line.clone()));
    }

    // TODO: looting

    play_state.set(PlayState::Exploring);
    commands.trigger(PlayerContinued { from: todo!() });
}

fn destroy(mut commands: Commands) {
    commands.remove_resource::<TurnOrder>();
    commands.remove_resource::<CombatLog>();
    commands.remove_resource::<CombatState>();
}
