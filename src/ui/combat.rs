use crate::prelude::*;
use bevy::ecs::relationship::RelatedSpawner;
use bevy::prelude::*;

use crate::game::combat::in_combat_phase;
use crate::game::combat::resources::AwaitingPlayerAttackTarget;
use crate::game::combat::resources::CombatLogAttack;
use crate::game::combat::resources::CombatLogDefend;
use crate::game::combat::resources::CombatLogResult;
use crate::game::combat::resources::HoveredAttackTarget;
use crate::game::components::CombatSlot;
use crate::ui::layout::GameArea;
use crate::ui::layout::HudAreaBottomCenter;
use bevy::ecs::relationship::RelatedSpawnerCommands;

const BORDER_IDLE: Color = Color::srgb(1., 1., 1.);
const BORDER_ACTIVE: Color = Color::srgb(1., 0.85, 0.2);

#[derive(Component)]
pub struct PlayerCombatButtonContainer;

#[derive(Component)]
pub struct PlayerAttackTargetButtonContainer;

#[derive(Component)]
pub struct CombatRoot;

#[derive(Component)]
pub struct CombatLogContainer;

#[derive(Component)]
pub struct CombatLogScrollArea;

#[derive(Component)]
pub struct CombatantContainer;

#[derive(Component)]
pub struct TurnOrderContainer;

#[derive(Component)]
pub struct EnemyCombatantContainer;

#[derive(Component)]
pub struct EnemyCombatantContent;

#[derive(Component)]
pub struct PlayerCombatantContainer;

#[derive(Component)]
pub struct PlayerCombatantContent;

#[derive(Component)]
pub struct TurnOrderContent;

#[derive(Component)]
pub struct LeavingContentContainer;

#[derive(Component)]
pub struct CombatantPanel(pub Entity);

#[derive(Component)]
pub struct CombatantTurnIcon(pub Entity);

pub struct CombatUIPlugin;

impl Plugin for CombatUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(PlayState::InCombat),
            (
                init_layout,
                init_log,
                init_enemies,
                init_player,
                init_turn_order,
            )
                .chain(),
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
            draw_enemy_stats.run_if(in_state(PlayState::InCombat)),
        );

        app.add_systems(
            Update,
            draw_player_stats.run_if(in_state(PlayState::InCombat)),
        );

        app.add_systems(
            Update,
            draw_turn_order.run_if(in_state(PlayState::InCombat)),
        );

        app.add_systems(
            Update,
            highlight_active_combatant.run_if(in_state(PlayState::InCombat)),
        );

        app.add_systems(
            Update,
            highlight_active_combatant_icon.run_if(in_state(PlayState::InCombat)),
        );

        app.add_systems(
            Update,
            player_action_buttons.run_if(in_state(PlayState::InCombat)),
        );

        app.add_systems(
            Update,
            player_attack_target_buttons.run_if(in_state(PlayState::InCombat)),
        );

        app.add_systems(
            Update,
            spawn_leaving_combat.run_if(
                in_state(PlayState::InCombat).and(in_combat_phase(CombatPhase::LeavingCombat)),
            ),
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
            // Combatants + Turn Indicator
            (
                CombatantContainer,
                Name::new("Combatant Container"),
                Node {
                    flex_grow: 1.,
                    flex_basis: Val::Px(0.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.),
                    ..default()
                },
                children![
                    // Enemies + Player
                    (
                        Name::new("Combatant Content"),
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
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(8.),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
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
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(8.),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                }
                            )
                        ]
                    ),
                    // Turn Order
                    (
                        TurnOrderContainer,
                        Name::new("Turn Order Container"),
                        Node {
                            flex_shrink: 0.,
                            height: Val::Percent(100.),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                    ),
                ]
            )
        ],
    ));
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

    commands.spawn((
        EnemyCombatantContent,
        Name::new("Enemy Combatant Content"),
        ChildOf(container.entity()),
        Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            column_gap: Val::Px(16.),
            ..default()
        },
    ));
}

fn init_player(
    mut commands: Commands,
    container: Single<Entity, With<PlayerCombatantContainer>>,
    player: Single<Entity, With<Player>>,
    fonts: Res<FontAssets>,
) {
    commands.spawn((
        Name::new("Player Combatant Area Title"),
        Text::new("You"),
        ChildOf(container.entity()),
        fonts.ui_font.clone(),
        fonts.ui_color,
        Node {
            align_self: AlignSelf::Center,
            ..default()
        },
    ));

    commands.spawn((
        PlayerCombatantContent,
        CombatantPanel(*player),
        Name::new("Player Combatant Content"),
        ChildOf(container.entity()),
        Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            column_gap: Val::Px(16.),
            border: UiRect::all(Val::Px(4.)),
            ..default()
        },
        BorderColor::all(Color::srgb(1., 1., 1.)),
    ));
}

fn init_turn_order(
    mut commands: Commands,
    container: Single<Entity, With<TurnOrderContainer>>,
    fonts: Res<FontAssets>,
) {
    commands.spawn((
        Name::new("Turn Order Title"),
        Text::new("Order"),
        Tooltip::basic("The turn order for this combat. Higher attacks first."),
        ChildOf(container.entity()),
        fonts.ui_font.clone(),
        fonts.ui_color,
        Node {
            align_self: AlignSelf::Center,
            ..default()
        },
    ));

    commands.spawn((
        TurnOrderContent,
        Name::new("Turn Order Content"),
        ChildOf(container.entity()),
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(8.),
            padding: UiRect::all(Val::Px(8.)),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BorderColor::all(Color::srgb(1., 1., 1.)),
    ));
}

fn draw_combat_log(
    mut commands: Commands,
    log: Res<CombatLog>,
    container: Single<Entity, With<CombatLogScrollArea>>,
    enemies: Query<(&DisplayName, &EnemyId), With<Enemy>>,
    player: Single<Entity, With<Player>>,
    fonts: Res<FontAssets>,
    enemy_icons: Res<EnemyIconAssets>,
    ui_icons: Res<UiIconAssets>,
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
                            fonts.ui_color,
                        ));
                    }
                    CombatLogLine::Attack(attack) => {
                        let CombatLogAttack {
                            attack_name,
                            attack_type,
                            attack_damage,
                            from,
                            to,
                            .. // TODO: probably want to show damage_type in UI
                        } = attack;

                        // Container
                        c.spawn(
                            combat_log_container(Color::srgb(1.0, 0.1, 0.1))
                        )
                        .with_children(|container| {
                            // From
                            let (from_name, from_icon) = match enemies.get(*from) {
                                Ok((name, id)) => (name.0.clone(), id.0.clone()),

                                Err(_) if player.entity() == *from => {
                                    (String::from("You"), String::from("player"))
                                }

                                Err(_) => {
                                    panic!("combat log attack source was neither enemy nor player: {from:?}");
                                }
                            };

                            draw_portrait(container, &enemy_icons, &from_name, &from_icon);

                            // Attack Name
                            container.spawn((
                                Text::new(format!("{} used {}", from_name, attack_name)),
                                fonts.ui_font.clone(),
                                fonts.ui_color,
                                Node {
                                    flex_grow: 1.,
                                    position_type: PositionType::Relative,
                                    ..default()
                                },
                            ));

                            let move_type_icon_id = match attack_type {
                                AttackType::Basic => "attack",
                                AttackType::Special => "special",
                            };

                            let Some(move_type_icon) = ui_icons.icons.get(move_type_icon_id) else {
                                return;
                            };

                            container.spawn((
                                Node {
                                    flex_direction: FlexDirection::Row,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    column_gap: Val::Px(4.),
                                    ..default()
                                },
                                children![
                                    (
                                        ImageNode::new(move_type_icon.clone()),
                                        Node {
                                            width: Val::Px(32.),
                                            height: Val::Px(32.),
                                            ..default()
                                        },
                                        Pickable::IGNORE,
                                    ),
                                    (
                                        Text::new(attack_damage.to_string()),
                                        fonts.ui_font.clone(),
                                        TextColor(Color::WHITE),
                                        Node {
                                            position_type: PositionType::Relative,
                                            bottom: Val::Px(2.), // Bump font up slightly
                                            ..default()
                                        },
                                    )
                                ],
                            ));

                            // To
                            let (to_name, to_icon) = match enemies.get(*to) {
                                Ok((name, id)) => (name.0.clone(), id.0.clone()),

                                Err(_) if player.entity() == *to => {
                                    (String::from("You"), String::from("player"))
                                }

                                Err(_) => {
                                    panic!("combat log attack target was neither enemy nor player: {from:?}");
                                }
                            };

                            draw_portrait(container, &enemy_icons, &to_name, &to_icon);
                        });
                    }
                    CombatLogLine::Effect(_) => {
                        // TODO: Not implemented effects yet.
                    },
                    CombatLogLine::Defend(CombatLogDefend { entity, potency }) => {
                        // Container
                        c.spawn(combat_log_container(Color::srgb(0.1, 0.1, 1.0)))
                        .with_children(|container| {
                            // From
                            let (entity_name, entity_icon) = match enemies.get(*entity) {
                                Ok((name, id)) => (name.0.clone(), id.0.clone()),

                                Err(_) if player.entity() == *entity => {
                                    (String::from("You"), String::from("player"))
                                }

                                Err(_) => {
                                    panic!("combat log defend source was neither enemy nor player: {entity:?}");
                                }
                            };

                            draw_portrait(container, &enemy_icons, &entity_name, &entity_icon);

                            // Defend Text
                            container.spawn((
                                Text::new(format!("{} prepares for an attack", entity_name)),
                                fonts.ui_font.clone(),
                                fonts.ui_color,
                                Node {
                                    flex_grow: 1.,
                                    position_type: PositionType::Relative,
                                    ..default()
                                },
                            ));

                            let Some(defend_icon) = ui_icons.icons.get("defend") else {
                                return;
                            };

                            container.spawn((
                                Node {
                                    flex_direction: FlexDirection::Row,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    column_gap: Val::Px(4.),
                                    ..default()
                                },
                                children![
                                    (
                                        ImageNode::new(defend_icon.clone()),
                                        Node {
                                            width: Val::Px(32.),
                                            height: Val::Px(32.),
                                            ..default()
                                        },
                                        Pickable::IGNORE,
                                    ),
                                    (
                                        Text::new(potency.to_string()),
                                        fonts.ui_font.clone(),
                                        TextColor(Color::WHITE),
                                        Node {
                                            position_type: PositionType::Relative,
                                            bottom: Val::Px(2.), // Bump font up slightly
                                            ..default()
                                        },
                                    )
                                ],
                            ));
                        });
                    },
                    CombatLogLine::CombatResult(CombatLogResult { message, player_won }) => {
                        let container_colour = match player_won {
                            true => Color::srgb(0.1, 0.1, 1.0),
                            false => Color::srgb(1.0, 0.1, 0.1),
                        };

                        c.spawn(combat_log_container(container_colour))
                            .with_children(|container| {
                                container.spawn((
                                    Text::new(message),
                                    fonts.ui_font.clone(),
                                    fonts.ui_color,
                                    Node {
                                        flex_grow: 1.,
                                        position_type: PositionType::Relative,
                                        ..default()
                                    },
                                ));
                            });
                    },
                }
            }
        });
}

fn combat_log_container(border_colour: Color) -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(8.),
            justify_content: JustifyContent::Stretch,
            border: UiRect::all(Val::Px(2.)),
            padding: UiRect::all(Val::Px(8.)),
            margin: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BorderColor::all(border_colour),
    )
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

fn draw_enemy_stats(
    mut commands: Commands,
    changed: Query<
        (),
        (
            With<Enemy>,
            Or<(
                Changed<Health>,
                Changed<MaxHealth>,
                Changed<EffectiveStats>,
                Changed<MovePlan>,
                Added<Dead>,
            )>,
        ),
    >,
    enemies: Query<
        (
            Entity,
            &Health,
            &MaxHealth,
            &Stats,
            &EffectiveStats,
            &DisplayName,
            &EnemyId,
            &MoveSet,
            &MovePlan,
            Option<&Dead>,
            &CombatSlot,
        ),
        With<Enemy>,
    >,
    panel: Single<Entity, With<EnemyCombatantContent>>,
    fonts: Res<FontAssets>,
    ui_icons: Res<UiIconAssets>,
    enemy_icons: Res<EnemyIconAssets>,
) {
    if changed.is_empty() {
        return;
    }

    // Sort by CombatSlot
    let mut ordered: Vec<_> = enemies.iter().collect();
    ordered.sort_by_key(|enemy| enemy.10.0);

    commands
        .entity(*panel)
        .despawn_children()
        .with_children(|panel| {
            for enemy in ordered {
                let (
                    entity,
                    health,
                    max,
                    base_stats,
                    effective_stats,
                    name,
                    id,
                    move_set,
                    move_plan,
                    dead,
                    ..,
                ) = enemy;

                panel
                    .spawn((
                        CombatantPanel(entity),
                        Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(8.),
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(4.)),
                            ..default()
                        },
                        BorderColor::all(Color::srgb(1., 1., 1.)),
                    ))
                    .with_children(|enemy_panel| {
                        // Top: Name + Icon
                        enemy_panel
                            .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::new(
                                    Val::Px(8.),
                                    Val::Px(8.),
                                    Val::Px(8.),
                                    Val::Px(0.),
                                ),
                                column_gap: Val::Px(8.),
                                ..default()
                            })
                            .with_children(|row| {
                                draw_portrait(row, &enemy_icons, &name.0, &id.0);

                                row.spawn((
                                    Text::new(name.0.clone()),
                                    fonts.ui_font.clone(),
                                    fonts.ui_color,
                                    Node {
                                        align_self: AlignSelf::Center,
                                        ..default()
                                    },
                                ));
                            });

                        // Bottom: HP + Stats
                        enemy_panel
                            .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(8.),
                                padding: UiRect::horizontal(Val::Px(8.)),
                                justify_content: JustifyContent::Center,
                                ..default()
                            })
                            .with_children(|stats_area| {
                                draw_hp(stats_area, health.0, max.0, &fonts);
                                draw_stats(
                                    stats_area,
                                    effective_stats.0,
                                    *base_stats,
                                    &ui_icons,
                                    &fonts,
                                );
                            });

                        // Moves
                        draw_enemy_moves(enemy_panel, move_set, move_plan, &fonts, &ui_icons);

                        // Dead overlay
                        if dead.is_some() {
                            enemy_panel.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    top: Val::Px(0.),
                                    left: Val::Px(0.),
                                    width: Val::Percent(100.),
                                    height: Val::Percent(100.),
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0., 0., 0., 0.6)),
                                GlobalZIndex(1),
                                Pickable::IGNORE,
                            ));
                        }
                    });
            }
        });
}

fn move_display(enemy_move: &EnemyMove) -> (String, i32, &str) {
    match enemy_move {
        EnemyMove::BasicAttack { name, potency, .. } => (name.clone(), *potency, "attack"),
        EnemyMove::SpecialAttack { name, potency, .. } => (name.clone(), *potency, "special"),
        EnemyMove::Defend { potency, .. } => ("Defend".to_string(), *potency, "defend"),
    }
}

fn draw_enemy_moves(
    parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
    move_set: &MoveSet,
    move_plan: &MovePlan,
    font_store: &Res<FontAssets>,
    ui_icons: &Res<UiIconAssets>,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_self: AlignSelf::Stretch,
            align_items: AlignItems::Center,
            row_gap: Val::Px(4.),
            ..default()
        })
        .with_children(|list| {
            // Divider
            list.spawn((
                Node {
                    width: Val::Percent(100.),
                    height: Val::Px(2.),
                    ..default()
                },
                BackgroundColor(Color::srgb(1., 1., 1.)),
            ));

            // Wrapper around moves for padding
            list.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                align_items: AlignItems::Center,
                row_gap: Val::Px(4.),
                padding: UiRect {
                    left: Val::Px(4.),
                    right: Val::Px(4.),
                    bottom: Val::Px(4.),
                    ..default()
                },
                ..default()
            })
            .with_children(|moves| {
                // Title
                moves.spawn((
                    Text::new("Upcoming Moves"),
                    font_store.ui_font.clone().with_font_size(20.),
                    font_store.ui_color,
                ));

                // Moves
                for (position, move_index) in move_plan.queue.iter().enumerate() {
                    let Some(enemy_move) = move_set.0.get(*move_index) else {
                        continue;
                    };
                    let (name, potency, icon_id) = move_display(enemy_move);

                    let Some(icon) = ui_icons.icons.get(icon_id) else {
                        return;
                    };

                    let bg_alpha = (move_plan.queue.len() - position) as f32 * 0.02;

                    moves
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Row,
                                align_self: AlignSelf::Stretch,
                                align_items: AlignItems::Center,
                                padding: UiRect::axes(Val::Px(8.), Val::Px(2.)),
                                column_gap: Val::Px(8.),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(1., 1., 1., bg_alpha)),
                        ))
                        .with_children(|row| {
                            // Move name
                            row.spawn((
                                Text::new(name),
                                font_store.ui_font.clone(),
                                font_store.ui_color,
                                Node {
                                    flex_grow: 1.,
                                    position_type: PositionType::Relative,
                                    bottom: Val::Px(2.), // Bump font up slightly
                                    ..default()
                                },
                            ));

                            // Move potency + icon
                            row.spawn((
                                Node {
                                    flex_direction: FlexDirection::Row,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    column_gap: Val::Px(4.),
                                    ..default()
                                },
                                children![
                                    (
                                        ImageNode::new(icon.clone()),
                                        Node {
                                            width: Val::Px(32.),
                                            height: Val::Px(32.),
                                            ..default()
                                        },
                                        Pickable::IGNORE,
                                    ),
                                    (
                                        Text::new(potency.to_string()),
                                        font_store.ui_font.clone(),
                                        TextColor(Color::WHITE),
                                        Node {
                                            position_type: PositionType::Relative,
                                            bottom: Val::Px(2.), // Bump font up slightly
                                            ..default()
                                        },
                                    )
                                ],
                            ));
                        });
                }
            });
        });
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
    panel: Single<Entity, With<PlayerCombatantContent>>,
    fonts: Res<FontAssets>,
    icons: Res<UiIconAssets>,
) {
    let (health, max, stats) = *player;

    commands
        .entity(*panel)
        .despawn_children()
        .with_children(|panel| {
            draw_hp(panel, health.0, max.0, &fonts);
            // Passing EffectiveStats for both 'current' and 'base'
            // because I don't really want the player stats to be highlighted when buffed
            // as it includes buffs from equipment atm and that's not 'temporary'
            // so TODO: pass just the 'bonus stats from buffs' to decide to highlight stats.
            draw_stats(panel, stats.0, stats.0, &icons, &fonts);
        });
}

fn draw_turn_order(
    mut commands: Commands,
    turn_order: Res<TurnOrder>,
    panel: Single<Entity, With<TurnOrderContent>>,
    enemies: Query<(Entity, &DisplayName, &EnemyId, Option<&Dead>), With<Enemy>>,
    player: Single<Entity, With<Player>>,
    enemy_icons: Res<EnemyIconAssets>,
) {
    if !turn_order.is_changed() {
        return;
    }

    let mut combatants = vec![(player.entity(), String::from("You"), String::from("player"))];

    combatants.extend(
        enemies
            .iter()
            .filter(|c| c.3.is_none()) // Skip dead
            .map(|(e, name, id, _)| (e, name.0.clone(), id.0.clone())),
    );

    commands
        .entity(*panel)
        .despawn_children()
        .with_children(|p| {
            for entity in turn_order.queue.iter() {
                let Some((e, name, id)) = combatants.iter().find(|c| c.0 == *entity) else {
                    continue; // Dead or removed; No need to draw
                };

                draw_portrait(p, &enemy_icons, &name, &id).insert(CombatantTurnIcon(*e));
            }
        });
}

fn highlight_active_combatant(
    turn_order: Res<TurnOrder>,
    hovered: Res<HoveredAttackTarget>,
    mut panels: Query<(&CombatantPanel, &mut BorderColor)>,
) {
    let active = turn_order.queue.get(turn_order.cursor).copied();

    for (combatant, mut border) in &mut panels {
        let should_highlight = Some(combatant.0) == active || Some(combatant.0) == hovered.0;

        let colour = if should_highlight {
            BORDER_ACTIVE
        } else {
            BORDER_IDLE
        };

        let desired = BorderColor::all(colour);

        if *border != desired {
            *border = desired;
        }
    }
}

fn highlight_active_combatant_icon(
    turn_order: Res<TurnOrder>,
    mut panels: Query<(&CombatantTurnIcon, &mut BorderColor)>,
) {
    let active = turn_order.queue.get(turn_order.cursor).copied();

    for (combatant, mut border) in &mut panels {
        let desired = BorderColor::all(if Some(combatant.0) == active {
            BORDER_ACTIVE
        } else {
            BORDER_IDLE
        });

        if *border != desired {
            *border = desired;
        }
    }
}

fn draw_hp(
    parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
    current: i32,
    max: i32,
    font_store: &Res<FontAssets>,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(8.),
            padding: UiRect::all(Val::Px(8.)),
            ..default()
        })
        .with_children(|hp| {
            // Current HP
            hp.spawn((
                Text::new(current.to_string()),
                Tooltip::basic("Current Health"),
                font_store.ui_font.clone(),
                font_store.ui_color,
            ));

            // Divider
            hp.spawn((
                Node {
                    width: Val::Percent(80.),
                    height: Val::Px(2.),
                    ..default()
                },
                BackgroundColor(Color::srgb(1., 1., 1.)),
            ));

            // Total HP
            hp.spawn((
                Text::new(max.to_string()),
                Tooltip::basic("Maximum Health"),
                font_store.ui_font.clone(),
                font_store.ui_color,
            ));
        });
}

fn draw_stat(
    parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
    label: &str,
    current_value: String,
    base_value: String,
    icon_key: &str,
    icon_store: &Res<UiIconAssets>,
    font_store: &Res<FontAssets>,
) {
    let Some(icon) = icon_store.icons.get(icon_key) else {
        return;
    };

    // Highlight stats that are buffed
    let text_colour = if current_value > base_value {
        TextColor(Color::srgb(0.1, 1.0, 0.1))
    } else {
        font_store.ui_color
    };

    parent.spawn((
        Tooltip::basic(label), // TODO: Detailed breakdown (base + buffs + equip)
        Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(4.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![
            (
                Text::new(current_value),
                font_store.ui_font.clone(),
                text_colour,
                Pickable::IGNORE
            ),
            (
                ImageNode::new(icon.clone()),
                Node {
                    width: Val::Px(32.),
                    height: Val::Px(32.),
                    ..default()
                },
                Pickable::IGNORE
            )
        ],
    ));
}

fn draw_stats(
    parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
    current_stats: Stats,
    base_stats: Stats,
    icon_store: &Res<UiIconAssets>,
    font_store: &Res<FontAssets>,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(4.),
            padding: UiRect::all(Val::Px(8.)),
            ..default()
        })
        .with_children(|stats_wrapper| {
            stats_wrapper
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(16.),
                    ..default()
                })
                .with_children(|row| {
                    draw_stat(
                        row,
                        "Strength",
                        current_stats.strength.to_string(),
                        base_stats.strength.to_string(),
                        "strength",
                        icon_store,
                        font_store,
                    );
                    draw_stat(
                        row,
                        "Agility",
                        current_stats.agility.to_string(),
                        base_stats.agility.to_string(),
                        "agility",
                        icon_store,
                        font_store,
                    );
                    draw_stat(
                        row,
                        "Intelligence",
                        current_stats.intelligence.to_string(),
                        base_stats.intelligence.to_string(),
                        "intelligence",
                        icon_store,
                        font_store,
                    );
                });

            stats_wrapper
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(16.),
                    ..default()
                })
                .with_children(|row| {
                    draw_stat(
                        row,
                        "Speed",
                        current_stats.speed.to_string(),
                        base_stats.speed.to_string(),
                        "speed",
                        icon_store,
                        font_store,
                    );
                    draw_stat(
                        row,
                        "Armour",
                        current_stats.armour.to_string(),
                        base_stats.armour.to_string(),
                        "armour",
                        icon_store,
                        font_store,
                    );
                });
        });
}

fn draw_portrait<'a>(
    parent: &'a mut RelatedSpawnerCommands<'_, ChildOf>,
    enemy_icons: &Res<EnemyIconAssets>,
    name: &String,
    icon_id: &String,
) -> EntityCommands<'a> {
    let Some(icon) = enemy_icons.icons.get(icon_id.as_str()) else {
        panic!("No icon!");
    };

    parent.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(2.)),
            border_radius: BorderRadius::all(Val::Percent(100.)),
            padding: UiRect::all(Val::Px(4.)),
            ..default()
        },
        Tooltip::basic(name.clone()),
        BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
        BorderColor::all(Color::srgb(1., 1., 1.)),
        children![(
            ImageNode::new(icon.clone()),
            Node {
                width: Val::Px(32.),
                height: Val::Px(32.),
                ..default()
            },
            Pickable::IGNORE,
        )],
    ))
}

fn player_action_buttons(
    mut commands: Commands,
    awaiting_action: Res<AwaitingPlayerAction>,
    awaiting_target: Res<AwaitingPlayerAttackTarget>,
    hud_area: Single<Entity, With<HudAreaBottomCenter>>,
    existing: Query<Entity, With<PlayerCombatButtonContainer>>,
) {
    if !awaiting_action.is_changed() && !awaiting_target.is_changed() {
        return;
    }

    // Hide when picking a target
    let show = awaiting_action.0 && !awaiting_target.0;

    if show {
        if !existing.is_empty() {
            // Already open
            return;
        }

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
            .observe(
                |_: On<Pointer<Click>>, mut attack_state: ResMut<AwaitingPlayerAttackTarget>| {
                    attack_state.0 = true;
                },
            );

        commands
            .spawn((button("Defend"), ChildOf(button_container)))
            .observe(|_: On<Pointer<Click>>, mut commands: Commands| {
                commands.trigger(PlayerCombatAction::Defend);
            });

        // commands
        //     .spawn((button("Special"), ChildOf(button_container)))
        //     .observe(|_: On<Pointer<Click>>, mut commands: Commands| {
        //         todo!();
        //     });

        // commands
        //     .spawn((button("Other"), ChildOf(button_container)))
        //     .observe(|_: On<Pointer<Click>>, mut commands: Commands| {
        //         todo!();
        //     });
    } else {
        for e in &existing {
            commands.entity(e).despawn();
        }
    }
}

fn player_attack_target_buttons(
    mut commands: Commands,
    awaiting_attack_target: Res<AwaitingPlayerAttackTarget>,
    hud_area: Single<Entity, With<HudAreaBottomCenter>>,
    existing: Query<Entity, With<PlayerAttackTargetButtonContainer>>,
    enemies: Query<(Entity, &DisplayName, Option<&Dead>, &CombatSlot), With<Enemy>>,
    mut hovered: ResMut<HoveredAttackTarget>,
) {
    if !awaiting_attack_target.is_changed() {
        return;
    }

    if awaiting_attack_target.0 {
        let button_container = commands
            .spawn((
                PlayerAttackTargetButtonContainer,
                Name::new("Attack Target Buttons Container"),
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

        // Sort by CombatSlot
        let mut ordered: Vec<_> = enemies.iter().collect();
        ordered.sort_by_key(|enemy| enemy.3.0);

        for (enemy_entity, enemy_name, enemy_dead, ..) in ordered {
            if enemy_dead.is_some() {
                continue;
            }

            commands
                .spawn((button(enemy_name.0.clone()), ChildOf(button_container)))
                .observe(move |_: On<Pointer<Click>>, mut commands: Commands| {
                    commands.trigger(PlayerCombatAction::Attack(enemy_entity));
                })
                .observe(
                    move |_: On<Pointer<Over>>, mut hovered: ResMut<HoveredAttackTarget>| {
                        hovered.0 = Some(enemy_entity);
                    },
                )
                .observe(
                    move |_: On<Pointer<Out>>, mut hovered: ResMut<HoveredAttackTarget>| {
                        if hovered.0 == Some(enemy_entity) {
                            hovered.0 = None;
                        }
                    },
                );
        }

        commands
            .spawn((button("Back"), ChildOf(button_container)))
            .observe(
                move |_: On<Pointer<Click>>,
                      mut attack_state: ResMut<AwaitingPlayerAttackTarget>,
                      mut action_state: ResMut<AwaitingPlayerAction>| {
                    attack_state.0 = false;
                    action_state.0 = true;
                },
            );
    } else {
        for e in &existing {
            commands.entity(e).try_despawn();
        }

        if hovered.0.is_some() {
            hovered.0 = None;
        }
    }
}

fn spawn_leaving_combat(
    mut commands: Commands,
    combat_state: Res<CombatState>,
    existing: Query<(), With<LeavingContentContainer>>,
    enemies: Query<&Gold, With<Enemy>>,
    font_store: Res<FontAssets>,
) {
    if !combat_state.is_changed() {
        return;
    }

    if !existing.is_empty() {
        return;
    }

    let title = if combat_state.result == CombatResult::PlayerWon {
        "Victory"
    } else {
        "Defeat"
    };

    let total_gold = enemies.iter().fold(0, |total, enemy| total + enemy.0);
    let ui_font = font_store.ui_font.clone();
    let ui_color = font_store.ui_color;

    commands.spawn((
        LeavingContentContainer,
        DespawnOnExit(PlayState::InCombat),
        panel(
            PanelProps::new(title).unclosable(),
            SpawnWith(move |p: &mut RelatedSpawner<ChildOf>| {
                p.spawn(Node {
                    width: Val::Percent(100.),
                    flex_grow: 1.,
                    flex_direction: FlexDirection::Column,
                    ..default()
                })
                .with_children(|p| {
                    build_combat_end_content(p, total_gold, ui_font, ui_color);
                });
            }),
        ),
    ));

    // TODO: looting
    //
    // commands.trigger(PlayerDied {
    //     reason: DeathReason::NoHealth,
    // });
}

fn build_combat_end_content(
    parent: &mut RelatedSpawner<'_, ChildOf>,
    total_gold: u32,
    ui_font: TextFont,
    ui_color: TextColor,
) {
    parent.spawn((Text::new(format!("{} Gold", total_gold)), ui_font, ui_color));

    parent.spawn(button("Continue")).observe(
        move |_: On<Pointer<Click>>,
              mut commands: Commands,
              query: Single<(Entity, &CurrentArea), With<Player>>,
              areas: Query<&AreaId, With<Area>>,
              mut play_state: ResMut<NextState<PlayState>>| {
            let (player, current_area) = *query;
            let player_entity = player.entity();

            let Ok(area_id) = areas.get(current_area.entity()) else {
                // Area not found - shouldn't happen
                return;
            };

            commands.trigger(GiveGold {
                amount: total_gold,
                beneficiary: player_entity,
            });

            play_state.set(PlayState::Exploring);

            commands.trigger(PlayerContinued {
                from: area_id.clone(),
            });
        },
    );
}
