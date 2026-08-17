use crate::prelude::*;
use bevy::prelude::*;

use crate::ui::interaction::image_tint::ImageTint;
use crate::ui::layout::HudAreaBottomRight;
use crate::world::equipment::EquipItemExt;
use bevy::ecs::relationship::RelatedSpawner;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::input::common_conditions::input_just_pressed;

const ITEM_BORDER_COLOUR: Color = Color::srgb(1., 1., 1.);
const ITEM_BORDER_SIZE: f32 = 4.;
const ITEM_SIZE: f32 = 100.;
const ITEM_PADDING: f32 = 10.;
const GRID_GAP: f32 = 10.;

const TOOLTIP_SIZE: f32 = 400.;
const TOOLTIP_ITEM_SIZE: f32 = 32.;

const TOOLTIP_LABEL_FONT_SIZE: f32 = 26.;
const TOOLTIP_DESC_FONT_SIZE: f32 = 18.;

const INVENTORY_BUTTON_ICON_SIZE: f32 = 64.;

#[derive(Component)]
pub struct InventoryPanel;

#[derive(States, Copy, Clone, PartialEq, Eq, Debug, Hash, Default)]
pub enum InventoryState {
    #[default]
    Closed,
    Open,
}

#[derive(Component)]
pub struct InventoryContent;

pub struct InventoryUIPlugin;

impl Plugin for InventoryUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InventoryState>();
        app.add_systems(OnEnter(GameState::Playing), button_init);
        app.add_systems(OnEnter(InventoryState::Open), spawn_inventory);
        app.add_systems(OnEnter(InventoryState::Closed), despawn_inventory);
        app.add_systems(Update, refresh_inventory);

        app.add_systems(
            Update,
            on_keybind_open_inventory.run_if(
                in_state(Pause(false))
                    .and(in_state(GameState::Playing))
                    .and(input_just_pressed(KeyCode::KeyI)),
            ),
        );

        app.add_systems(
            Update,
            despawn_inventory.run_if(
                in_state(Pause(false))
                    .and(in_state(GameState::Playing))
                    .and(in_state(InventoryState::Open))
                    .and(input_just_pressed(KeyCode::Escape)),
            ),
        );
    }
}

fn button_init(
    mut commands: Commands,
    icon_store: Res<UiIconAssets>,
    audio_store: Res<AudioAssets>,
    hud_area: Single<Entity, With<HudAreaBottomRight>>,
) {
    let Some(click_sfx) = audio_store.sfx.get("click") else {
        warn!("Failed to find click SFX. See: {:?}", audio_store.sfx);
        return;
    };

    let Some(inventory_icon) = icon_store.icons.get("inventory").cloned() else {
        warn!("Failed to find inventory icon. See: {:?}", icon_store.icons);
        return;
    };

    commands
        .spawn((
            Button,
            ImageTint::darken(Color::srgb(1., 1., 1.)),
            ClickSfx::from(click_sfx.clone()),
            Name::new("Inventory Button"),
            Tooltip::basic("Inventory"),
            ChildOf(hud_area.entity()),
            DespawnOnExit(GameState::Playing),
            GlobalZIndex(LAYER_HUD),
            Node {
                padding: UiRect::all(Val::Px(8.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            children![(
                Name::new("Inventory Button Icon"),
                Node {
                    width: Val::Px(INVENTORY_BUTTON_ICON_SIZE),
                    height: Val::Px(INVENTORY_BUTTON_ICON_SIZE),
                    ..default()
                },
                Pickable::IGNORE,
                ImageNode::new(inventory_icon),
            )],
        ))
        .observe(
            |_: On<Pointer<Click>>,
             state: Res<State<InventoryState>>,
             mut next: ResMut<NextState<InventoryState>>| {
                match state.get() {
                    InventoryState::Closed => next.set(InventoryState::Open),
                    InventoryState::Open => next.set(InventoryState::Closed),
                };
            },
        );
}

fn despawn_inventory(
    mut commands: Commands,
    panel: Single<Entity, With<InventoryPanel>>,
    state: Res<State<InventoryState>>,
) {
    match state.get() {
        InventoryState::Closed => {}
        InventoryState::Open => {
            commands
                .entity(*panel)
                .trigger(|p| DespawnPanel { entity: p });
        }
    }
}

fn on_keybind_open_inventory(
    mut commands: Commands,
    state: Res<State<InventoryState>>,
    mut next: ResMut<NextState<InventoryState>>,
    audio_store: Res<AudioAssets>,
) {
    match state.get() {
        InventoryState::Closed => {
            let Some(coin_sfx) = audio_store.sfx.get("coins") else {
                warn!("Failed to find coins SFX. See: {:?}", audio_store.sfx);
                return;
            };

            commands.trigger(PlaySfx::new(coin_sfx.clone()));
            next.set(InventoryState::Open);
        }
        InventoryState::Open => {}
    };
}

fn collect_rows(
    inventory: &Inventory,
    item_store: Res<ItemStore>,
    icon_assets: Res<UiIconAssets>,
) -> Vec<ItemRow> {
    inventory
        .iter()
        .filter_map(|item_stack| {
            let store_item = item_store.get(&item_stack.item_id.0)?;
            let count = item_stack.count;

            let stats = store_item
                .stats
                .map(|stats| {
                    stats
                        .non_zero_stats()
                        .into_iter()
                        .filter_map(|(label, value, icon)| {
                            let icon = icon_assets.icons.get(icon)?.clone();
                            Some(StatLine {
                                label: label.to_string(),
                                value,
                                icon,
                            })
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            Some(ItemRow {
                id: store_item.id.clone(),
                name: store_item.name.clone(),
                description: store_item.description.clone(),
                icon: store_item.icon.clone(),
                slot: store_item.slot,
                stats: stats,
                count: count,
            })
        })
        .collect()
}

fn build_inventory_content(
    parent: &mut RelatedSpawner<'_, ChildOf>,
    rows: Vec<ItemRow>,
    gold: u32,
    ui_font: TextFont,
    ui_color: TextColor,
    player_entity: Entity,
    health: i32,
    max_health: i32,
    stats: Vec<(String, Handle<Image>, i32)>,
) {
    let mut inventory_content = parent.spawn((
        Name::new("Inventory Content"),
        Node {
            flex_direction: FlexDirection::Row,
            width: Val::Percent(100.),
            flex_grow: 1.,
            column_gap: Val::Px(8.),
            ..default()
        },
    ));

    // Spawn left: The inventory
    inventory_content.with_children(|p| {
        p.spawn((
            Node {
                display: Display::Flex,
                width: Val::Percent(50.),
                flex_grow: 1.,
                padding: UiRect::all(Val::Px(20.)),
                ..default()
            },
            children![scroll_area({
                let ui_font = ui_font.clone();
                let ui_color = ui_color;

                move |p| {
                    p.spawn(inventory_grid()).with_children(|grid| {
                        for item in rows {
                            let mut invent_item =
                                grid.spawn(inventory_item(item.clone(), ui_font.clone(), ui_color));

                            invent_item.with_children(|tile| {
                                tile.spawn(item_icon(item.id.clone(), item.icon.clone()));

                                if item.count > 0 {
                                    tile.spawn(item_count(
                                        item.id.clone(),
                                        item.count.to_string(),
                                        ui_font.clone(),
                                        ui_color,
                                    ));
                                }
                            });

                            if item.slot.is_some() {
                                invent_item.observe(
                                    move |_: On<Pointer<Click>>, mut commands: Commands| {
                                        commands
                                            .entity(player_entity)
                                            .equip_from_inventory(item.id.clone());
                                    },
                                );
                            };
                        }
                    });
                }
            })],
        ));
    });

    // Spawn Right: player stats + equipment
    inventory_content.with_children(|p| {
        p.spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.),
                width: Val::Percent(50.),
                flex_grow: 1.,
                padding: UiRect::all(Val::Px(20.)),
                ..default()
            },
            children![
                equipment_status_bar(health, max_health, stats, ui_font.clone(), ui_color),
                equipment_slots()
            ],
        ));
    });

    // Bottom row
    parent.spawn((
        Node {
            display: Display::Flex,
            width: Val::Percent(100.),
            flex_shrink: 0.,
            padding: UiRect::all(Val::Px(10.)),
            ..default()
        },
        children![(
            Name::new("Gold Count"),
            Text::new(format!("{} Gold", gold)),
            ui_font.clone(),
            ui_color,
        )],
    ));
}

fn spawn_inventory(
    mut commands: Commands,
    mut inventory_state: ResMut<NextState<InventoryState>>,
    player: Single<(Entity, &Health, &MaxHealth, &EffectiveStats), With<Player>>,
    inventory: Single<&Inventory, With<Player>>,
    gold: Single<&Gold, With<Player>>,
    item_store: Res<ItemStore>,
    icon_store: Res<UiIconAssets>,
    fonts: Res<FontAssets>,
) {
    if inventory.is_empty() {
        inventory_state.set(InventoryState::Closed);
        return;
    }

    let gold = gold.0;

    let (player_entity, health, max_health, stats) = *player;
    let health = health.0;
    let max_health = max_health.0;

    let stats: Vec<(String, Handle<Image>, i32)> = [
        ("Strength", "strength", stats.0.strength),
        ("Agility", "agility", stats.0.agility),
        ("Intelligence", "intelligence", stats.0.intelligence),
        ("Speed", "speed", stats.0.speed),
        ("Armour", "armour", stats.0.armour),
    ]
    .into_iter()
    .filter_map(|(label, key, value)| {
        Some((label.to_string(), icon_store.icons.get(key)?.clone(), value))
    })
    .collect();

    let rows = collect_rows(&inventory, item_store, icon_store);

    let ui_font = fonts.ui_font.clone();
    let ui_color = fonts.ui_color;

    commands
        .spawn((
            InventoryPanel,
            panel(
                PanelProps::new("Inventory").sized(Val::Percent(80.), Val::Percent(70.)),
                SpawnWith(move |p: &mut RelatedSpawner<ChildOf>| {
                    p.spawn((
                        InventoryContent,
                        Node {
                            width: Val::Percent(100.),
                            flex_grow: 1.,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                    ))
                    .with_children(|content| {
                        build_inventory_content(
                            content,
                            rows,
                            gold,
                            ui_font,
                            ui_color,
                            player_entity,
                            health,
                            max_health,
                            stats,
                        );
                    });
                }),
            ),
        ))
        .observe(
            // If the panel is despawned, it means the inventory has been closed.
            // This allows the panel to close itself via it's close button.
            |_: On<DespawnPanel>, mut next: ResMut<NextState<InventoryState>>| {
                next.set(InventoryState::Closed);
            },
        );
}

fn refresh_inventory(
    mut commands: Commands,
    player: Single<
        (
            Entity,
            &Inventory,
            &Gold,
            &Health,
            &MaxHealth,
            &EffectiveStats,
        ),
        (
            With<Player>,
            Or<(
                Changed<Inventory>,
                Changed<Gold>,
                Changed<Equipment>,
                Changed<Health>,
                Changed<MaxHealth>,
                Changed<EffectiveStats>,
            )>,
        ),
    >,
    item_store: Res<ItemStore>,
    icon_store: Res<UiIconAssets>,
    fonts: Res<FontAssets>,
    content: Single<Entity, With<InventoryContent>>,
) {
    let (player_entity, inventory, gold, health, max_health, stats) = *player;

    let stats: Vec<(String, Handle<Image>, i32)> = [
        ("Strength", "strength", stats.0.strength),
        ("Agility", "agility", stats.0.agility),
        ("Intelligence", "intelligence", stats.0.intelligence),
        ("Speed", "speed", stats.0.speed),
        ("Armour", "armour", stats.0.armour),
    ]
    .into_iter()
    .filter_map(|(label, key, value)| {
        Some((label.to_string(), icon_store.icons.get(key)?.clone(), value))
    })
    .collect();

    let health = health.0;
    let max_health = max_health.0;
    let gold = gold.0;
    let ui_font = fonts.ui_font.clone();
    let ui_color = fonts.ui_color;

    let rows = collect_rows(inventory, item_store, icon_store);

    // Clear existing inventory items
    commands.entity(*content).despawn_children();

    // Replace with new items
    commands.entity(*content).insert(Children::spawn(SpawnWith(
        move |content: &mut RelatedSpawner<ChildOf>| {
            build_inventory_content(
                content,
                rows,
                gold,
                ui_font,
                ui_color,
                player_entity,
                health,
                max_health,
                stats,
            );
        },
    )));
}

const STAT_ICON_SIZE: f32 = 32.;

fn equipment_status_bar(
    health: i32,
    max_health: i32,
    stats: Vec<(String, Handle<Image>, i32)>,
    font: TextFont,
    color: TextColor,
) -> impl Bundle {
    (
        Name::new("Equipment Status Bar"),
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::End,
            column_gap: Val::Px(20.),
            padding: UiRect::all(Val::Px(15.)),
            ..default()
        },
        Children::spawn(SpawnWith(move |p: &mut RelatedSpawner<ChildOf>| {
            // Health
            p.spawn((
                Name::new("Health"),
                Node {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(8.),
                    padding: UiRect::all(Val::Px(8.)),
                    ..default()
                },
                // TODO: deduplicate with combat UI
                children![
                    // Current HP
                    (
                        Text::new(health.to_string()),
                        Tooltip::basic("Current Health"),
                        font.clone(),
                        color,
                    ),
                    // Divider
                    (
                        Node {
                            width: Val::Percent(80.),
                            height: Val::Px(2.),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(1., 1., 1.)),
                    ),
                    // Max HP
                    (
                        Text::new(max_health.to_string()),
                        Tooltip::basic("Maximum Health"),
                        font.clone(),
                        color,
                    )
                ],
            ));

            // Stats
            for (label, icon, value) in stats {
                p.spawn((
                    Tooltip::basic(label),
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(4.),
                        ..default()
                    },
                    children![
                        (
                            Node {
                                width: Val::Px(STAT_ICON_SIZE),
                                height: Val::Px(STAT_ICON_SIZE),
                                ..default()
                            },
                            ImageNode::new(icon),
                            Pickable::IGNORE
                        ),
                        (
                            font.clone(),
                            color,
                            Text::new(value.to_string()),
                            Pickable::IGNORE
                        ),
                    ],
                ));
            }
        })),
    )
}

fn equipment_slots() -> impl Bundle {
    ()
}

#[derive(Clone, Debug)]
struct StatLine {
    label: String,
    value: String,
    icon: Handle<Image>,
}

#[derive(Clone, Debug)]
struct ItemRow {
    id: String,
    name: String,
    description: String,
    count: u32,
    icon: Handle<Image>,
    slot: Option<EquipmentSlot>,
    stats: Vec<StatLine>,
}

fn inventory_grid() -> impl Bundle {
    (
        Name::new("Inventory Grid"),
        Node {
            display: Display::Grid,
            row_gap: Val::Px(GRID_GAP),
            column_gap: Val::Px(GRID_GAP),
            grid_template_columns: RepeatedGridTrack::px(4, ITEM_SIZE),
            grid_auto_rows: vec![GridTrack::px(ITEM_SIZE)],
            grid_auto_flow: GridAutoFlow::Column,
            ..default()
        },
    )
}

fn inventory_item(item: ItemRow, font: TextFont, color: TextColor) -> impl Bundle {
    let tooltip_label = if item.count > 0 {
        format!("{} ({})", item.name, item.count)
    } else {
        item.name.clone()
    };

    (
        Name::new(format!("Inventory Item '{}' (ID: {})", item.name, item.id)),
        Node {
            border: UiRect::all(Val::Px(ITEM_BORDER_SIZE)),
            padding: UiRect::all(Val::Px(ITEM_PADDING)),
            ..default()
        },
        BorderColor::all(ITEM_BORDER_COLOUR),
        Tooltip::new_sized(
            move |p| {
                build_tooltip(
                    p,
                    item.id.clone(),
                    tooltip_label.clone(),
                    item.description.clone(),
                    item.icon.clone(),
                    item.slot,
                    item.stats.clone(),
                    font.clone(),
                    color.clone(),
                );
            },
            TOOLTIP_SIZE,
        ),
    )
}

fn item_icon(item_id: String, item_icon: Handle<Image>) -> impl Bundle {
    (
        Name::new(format!("Inventory Item Icon (ID: {})", item_id)),
        ImageNode::new(item_icon),
        Pickable::IGNORE,
    )
}

fn item_count(item_id: String, count: String, font: TextFont, color: TextColor) -> impl Bundle {
    (
        Name::new(format!("Inventory Item Count (ID: {})", item_id)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.),
            right: Val::Px(10.),
            ..default()
        },
        font,
        color,
        Text::new(count),
        Pickable::IGNORE,
    )
}

fn build_tooltip(
    parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
    tooltip_id: String,
    label: String,
    description: String,
    icon: Handle<Image>,
    slot: Option<EquipmentSlot>,
    stats: Vec<StatLine>,
    font: TextFont,
    color: TextColor,
) {
    parent
        .spawn((
            Name::new(format!("Inventory Item Tooltip (ID: {})", tooltip_id)),
            // -- Column of toolip (1st row + description)
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.),
                ..default()
            },
        ))
        .with_children(|p| {
            // -- First row of tooltip (icon + label)
            p.spawn((Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.),
                align_items: AlignItems::Center,
                ..default()
            },))
                .with_children(|row| {
                    // Icon
                    row.spawn((
                        Node {
                            width: Val::Px(TOOLTIP_ITEM_SIZE),
                            height: Val::Px(TOOLTIP_ITEM_SIZE),
                            ..default()
                        },
                        ImageNode::new(icon),
                    ));

                    // Label
                    row.spawn((
                        font.clone().with_font_size(TOOLTIP_LABEL_FONT_SIZE),
                        color.clone(),
                        Text::new(label),
                    ));
                });

            // -- Description
            p.spawn((
                font.clone().with_font_size(TOOLTIP_DESC_FONT_SIZE),
                color,
                Text::new(description),
            ));

            // Slot
            if let Some(slot) = slot {
                p.spawn((
                    font.clone().with_font_size(TOOLTIP_DESC_FONT_SIZE),
                    color,
                    Text::new(slot.to_string()),
                ));
            }

            // Stats
            for stat in stats {
                p.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.),
                        ..default()
                    },
                    children![
                        (
                            Node {
                                width: Val::Px(16.),
                                height: Val::Px(16.),
                                ..default()
                            },
                            ImageNode::new(stat.icon),
                        ),
                        (
                            font.clone().with_font_size(TOOLTIP_DESC_FONT_SIZE),
                            color,
                            Text::new(format!("{} {}", stat.value, stat.label)),
                        )
                    ],
                ));
            }
        });
}
