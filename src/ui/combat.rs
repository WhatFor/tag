use crate::prelude::*;
use bevy::prelude::*;

use crate::ui::layout::GameArea;
use crate::ui::layout::HudAreaBottomCenter;

#[derive(Component)]
pub struct PlayerCombatButtonContainer;

#[derive(Component)]
pub struct CombatRoot;

#[derive(Component)]
pub struct CombatLogContainer;

#[derive(Component)]
pub struct CombatLogScrollArea;

#[derive(Component)]
pub struct CombatantContainer;

#[derive(Component)]
pub struct EnemyCombatantContainer;

#[derive(Component)]
pub struct PlayerCombatantContainer;

pub struct CombatUIPlugin;

impl Plugin for CombatUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(PlayState::InCombat),
            (init_layout, init_log, init_enemies, init_player).chain(),
        );

        app.add_systems(
            Update,
            draw_combat_log.run_if(in_state(PlayState::InCombat)),
        );

        app.add_systems(
            PostUpdate,
            autoscroll_combat_log
                .after(bevy::ui::UiSystems::Layout)
                .run_if(in_state(PlayState::InCombat)),
        );

        app.add_systems(
            Update,
            draw_player_stats.run_if(in_state(PlayState::InCombat)),
        );

        app.add_systems(
            Update,
            player_action_buttons.run_if(in_state(PlayState::InCombat)),
        );
    }
}

fn init_layout(mut commands: Commands, game_area: Single<Entity, With<GameArea>>) {
    commands.spawn((
        CombatRoot,
        Name::new("Combat Root"),
        ChildOf(game_area.entity()),
        DespawnOnExit(PlayState::InCombat),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(16.),
            ..default()
        },
        children![
            // Combat Log
            (
                CombatLogContainer,
                Name::new("Combat Log Container"),
                Node {
                    flex_grow: 1.,
                    flex_basis: Val::Px(0.),
                    height: Val::Percent(100.),
                    padding: UiRect::all(Val::Px(16.)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.),
                    ..default()
                }
            ),
            // Combatants
            (
                CombatantContainer,
                Name::new("Combatant Container"),
                Node {
                    flex_grow: 1.,
                    flex_basis: Val::Px(0.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(16.),
                    ..default()
                },
                children![
                    // Enemies
                    (
                        EnemyCombatantContainer,
                        Name::new("Enemy Combatant Container"),
                        Node {
                            width: Val::Percent(100.),
                            flex_grow: 1.,
                            flex_basis: Val::Px(0.),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(8.),
                            ..default()
                        }
                    ),
                    // Player
                    (
                        PlayerCombatantContainer,
                        Name::new("Player Combatant Container"),
                        Node {
                            width: Val::Percent(100.),
                            flex_grow: 1.,
                            flex_basis: Val::Px(0.),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        }
                    )
                ]
            )
        ],
    ));

    // let Ok(content) = all_area_content.get(area.0) else {
    //     return;
    // };

    // if let AreaContent::Combat {
    //     start_lines,
    //     enemy_ids,
    //     win_lines,
    //     lose_lines,
    // } = content
    // {
    //     for line in start_lines {
    //         info!(line);
    //     }
    // } else {
    //     warn!("Player is in combat state but not in a combat area!");
    // }
}

fn init_log(
    mut commands: Commands,
    container: Single<Entity, With<CombatLogContainer>>,
    fonts: Res<FontAssets>,
) {
    commands.spawn((
        Name::new("Scroll Area Title"),
        Text::new("Combat Log"),
        ChildOf(container.entity()),
        fonts.ui_font.clone(),
        fonts.ui_color,
        Node {
            align_self: AlignSelf::Center,
            ..default()
        },
    ));

    commands.spawn((
        ChildOf(container.entity()),
        scroll_area(|log| {
            log.spawn((
                CombatLogScrollArea,
                Node {
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
            ));
        }),
    ));
}

fn init_enemies(
    mut commands: Commands,
    container: Single<Entity, With<EnemyCombatantContainer>>,
    fonts: Res<FontAssets>,
) {
    commands.spawn((
        Name::new("Enemy Combatants Area Title"),
        Text::new("Foe"),
        ChildOf(container.entity()),
        fonts.ui_font.clone(),
        fonts.ui_color,
        Node {
            align_self: AlignSelf::Center,
            ..default()
        },
    ));
}

fn init_player(
    mut commands: Commands,
    container: Single<Entity, With<PlayerCombatantContainer>>,
    fonts: Res<FontAssets>,
) {
    commands.spawn((
        Name::new("Player Combatants Area Title"),
        Text::new("You"),
        ChildOf(container.entity()),
        fonts.ui_font.clone(),
        fonts.ui_color,
        Node {
            align_self: AlignSelf::Center,
            ..default()
        },
    ));
}

fn draw_combat_log(
    mut commands: Commands,
    log: Res<CombatLog>,
    container: Single<Entity, With<CombatLogScrollArea>>,
    fonts: Res<FontAssets>,
) {
    if !log.is_changed() {
        return;
    }

    commands
        .entity(*container)
        .despawn_children()
        .with_children(|c| {
            for line in &log.lines {
                match line {
                    CombatLogLine::Text(text) => {
                        c.spawn((
                            Text::new(text.clone()),
                            fonts.ui_font.clone(),
                            fonts.ui_color.clone(),
                        ));
                    }
                }
            }
        });
}

fn autoscroll_combat_log(
    content: Single<&ChildOf, With<CombatLogScrollArea>>,
    mut viewports: Query<(&mut ScrollPosition, &ComputedNode)>,
    mut last_height: Local<f32>,
) {
    let Ok((mut scroll, node)) = viewports.get_mut(content.parent()) else {
        return;
    };

    let content_height = node.content_size().y;

    if content_height > *last_height {
        scroll.0.y = (content_height - node.size().y).max(0.) * node.inverse_scale_factor();
    }

    *last_height = content_height;
}

fn draw_player_stats(
    mut commands: Commands,
    player: Single<
        (&Health, &MaxHealth, &EffectiveStats),
        (
            With<Player>,
            Or<(Changed<Health>, Changed<MaxHealth>, Changed<EffectiveStats>)>,
        ),
    >,
    panel: Single<Entity, With<PlayerCombatantContainer>>,
    fonts: Res<FontAssets>,
) {
    let (health, max, stats) = *player;

    // TODO: This replaces the entire content of the panel, including the title.

    commands
        .entity(*panel)
        .despawn_children()
        .with_children(|panel| {
            panel.spawn((
                Text::new(format!("HP: {}/{}", health.0, max.0)),
                fonts.ui_font.clone(),
                fonts.ui_color.clone(),
            ));
            // TODO: Draw stats
        });
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
