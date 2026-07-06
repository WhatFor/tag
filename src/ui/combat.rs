use crate::prelude::*;
use bevy::prelude::*;

use crate::ui::layout::HudAreaBottomCenter;

#[derive(Component)]
pub struct PlayerCombatButtonContainer;

pub struct CombatUIPlugin;

impl Plugin for CombatUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(PlayState::InCombat),
            start_combat.in_set(PausableSystems),
        );

        app.add_systems(
            Update,
            player_action_buttons.run_if(in_state(PlayState::InCombat)),
        );
    }
}

fn start_combat(
    mut commands: Commands,
    area: Single<&CurrentArea, With<Player>>,
    all_area_content: Query<&AreaContent, With<Area>>,
) {
    let Ok(content) = all_area_content.get(area.0) else {
        return;
    };

    if let AreaContent::Combat {
        start_lines,
        enemy_ids,
        win_lines,
        lose_lines,
    } = content
    {
        for line in start_lines {
            info!(line);
        }
    } else {
        warn!("Player is in combat state but not in a combat area!");
    }
}

fn player_action_buttons(
    mut commands: Commands,
    awaiting_player: Res<AwaitingPlayerAction>,
    hud_area: Single<Entity, With<HudAreaBottomCenter>>,
    existing: Query<Entity, With<PlayerCombatButtonContainer>>,
) {
    if !awaiting_player.is_changed() {
        return;
    }

    if awaiting_player.0 {
        let button_container = commands
            .spawn((
                PlayerCombatButtonContainer,
                Name::new("Combat Buttons Container"),
                ChildOf(hud_area.entity()),
                GlobalZIndex(LAYER_HUD),
                Node {
                    display: Display::Grid,
                    grid_template_columns: RepeatedGridTrack::flex(2, 1.0),
                    grid_template_rows: RepeatedGridTrack::flex(2, 1.0),
                    column_gap: Val::Px(8.),
                    row_gap: Val::Px(8.),
                    ..Default::default()
                },
                DespawnOnExit(PlayState::InCombat),
            ))
            .id();

        commands
            .spawn((button("Attack"), ChildOf(button_container)))
            .observe(|_: On<Pointer<Click>>, mut commands: Commands| {
                commands.trigger(PlayerCombatAction::Attack);
            });

        commands
            .spawn((button("Defend"), ChildOf(button_container)))
            .observe(|_: On<Pointer<Click>>, mut commands: Commands| {
                commands.trigger(PlayerCombatAction::Defend);
            });

        commands
            .spawn((button("Special"), ChildOf(button_container)))
            .observe(|_: On<Pointer<Click>>, mut commands: Commands| {
                todo!();
            });

        commands
            .spawn((button("Other"), ChildOf(button_container)))
            .observe(|_: On<Pointer<Click>>, mut commands: Commands| {
                todo!();
            });
    } else {
        for e in &existing {
            commands.entity(e).despawn();
        }
    }
}
