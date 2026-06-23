use crate::prelude::*;
use bevy::prelude::*;

use crate::ui::interaction::image_tint::ImageTint;
use crate::ui::layout::HudAreaBottomRight;
use crate::ui::widgets::panel::DespawnPanel;
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

pub struct InventoryUIPlugin;

impl Plugin for InventoryUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InventoryState>();
        app.add_systems(OnEnter(GameState::Playing), button_init);
        app.add_systems(OnEnter(InventoryState::Open), spawn_inventory);
        app.add_systems(OnEnter(InventoryState::Closed), despawn_inventory);

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
    icon_store: Res<IconAssets>,
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

fn spawn_inventory(
    mut commands: Commands,
    inventory: Single<&Inventory, With<Player>>,
    items: Query<(&ItemId, Option<&ItemStack>)>,
    item_store: Res<ItemStore>,
    fonts: Res<FontAssets>,
) {
    if inventory.is_empty() {
        return;
    }

    let rows: Vec<ItemRow> = inventory
        .iter()
        .filter_map(|&item_entity| {
            let (id, stack) = items.get(item_entity).ok()?;
            let store_item = item_store.get(&id.0)?;

            let count = match stack {
                Some(ItemStack(count)) => Some(count.clone()),
                None => None,
            };

            Some(ItemRow {
                id: store_item.id.clone(),
                name: store_item.name.clone(),
                description: store_item.description.clone(),
                icon: store_item.icon.clone(),
                count: count,
            })
        })
        .collect();

    let ui_font = fonts.ui_font.clone();
    let ui_color = fonts.ui_color.clone();

    commands
        .spawn((
            Panel::default("Inventory".into()),
            InventoryPanel,
            Name::new("Inventory Panel"),
        ))
        .observe(
            // If the panel is despawned, it means the inventory has been closed.
            // This allows the panel to close itself via it's close button.
            |_: On<DespawnPanel>, mut next: ResMut<NextState<InventoryState>>| {
                next.set(InventoryState::Closed);
            },
        )
        .with_children(|p| {
            p.spawn((
                Node {
                    display: Display::Flex,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    padding: UiRect::all(Val::Px(20.)),
                    ..default()
                },
                children![scroll_area(move |p| {
                    p.spawn(inventory_grid()).with_children(|grid| {
                        for item in rows {
                            grid.spawn(inventory_item(
                                item.clone(),
                                ui_font.clone(),
                                ui_color.clone(),
                            ))
                            .with_children(|tile| {
                                tile.spawn(item_icon(item.id.clone(), item.icon.clone()));

                                if let Some(count) = item.count.filter(|&c| c > 0) {
                                    tile.spawn(item_count(
                                        item.id.clone(),
                                        count.to_string(),
                                        ui_font.clone(),
                                        ui_color.clone(),
                                    ));
                                }
                            });
                        }
                    });
                })],
            ));
        });
}

#[derive(Clone, Debug)]
struct ItemRow {
    id: String,
    name: String,
    description: String,
    count: Option<u32>,
    icon: Handle<Image>,
}

fn inventory_grid() -> impl Bundle {
    (
        Name::new("Inventory Grid"),
        Node {
            display: Display::Grid,
            row_gap: Val::Px(GRID_GAP),
            column_gap: Val::Px(GRID_GAP),
            grid_template_columns: RepeatedGridTrack::px(8, ITEM_SIZE),
            grid_auto_rows: vec![GridTrack::px(ITEM_SIZE)],
            grid_auto_flow: GridAutoFlow::Column,
            ..default()
        },
    )
}

fn inventory_item(item: ItemRow, font: TextFont, color: TextColor) -> impl Bundle {
    let tooltip_label = match item.count {
        Some(count) => format!("{} ({})", item.name, count),
        None => item.name.clone(),
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
                p.spawn(tooltip(
                    item.id.clone(),
                    tooltip_label.clone(),
                    item.description.clone(),
                    item.icon.clone(),
                    font.clone(),
                    color.clone(),
                ));
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

fn tooltip(
    tooltip_id: String,
    tooltip_label: String,
    tooltip_desc: String,
    tooltip_icon: Handle<Image>,
    font: TextFont,
    color: TextColor,
) -> impl Bundle {
    (
        Name::new(format!("Inventory Item Tooltip (ID: {})", tooltip_id)),
        // -- Column of toolip (1st row + description)
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.),
            ..default()
        },
        children![
            (
                // -- First row of tooltip (icon + label)
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.),
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![
                    // -- Icon
                    (
                        Node {
                            width: Val::Px(TOOLTIP_ITEM_SIZE),
                            height: Val::Px(TOOLTIP_ITEM_SIZE),
                            ..default()
                        },
                        ImageNode::new(tooltip_icon),
                    ),
                    // -- Label
                    (
                        font.clone().with_font_size(TOOLTIP_LABEL_FONT_SIZE),
                        color.clone(),
                        Text::new(tooltip_label)
                    ),
                ],
            ),
            // -- Description
            (
                font.with_font_size(TOOLTIP_DESC_FONT_SIZE),
                color,
                Text::new(tooltip_desc)
            ),
        ],
    )
}
