use crate::prelude::*;
use bevy::prelude::*;

use crate::ui::widgets::panel::DespawnPanel;

pub struct InventoryUIPlugin;

const ITEM_BORDER_COLOUR: Color = Color::srgb(0.4, 0.4, 0.4);
const ITEM_SIZE: f32 = 100.;
const ITEM_PADDING: f32 = 10.;
const GRID_GAP: f32 = 10.;

#[derive(Component)]
pub struct InventoryPanel;

#[derive(States, Copy, Clone, PartialEq, Eq, Debug, Hash, Default)]
enum InventoryState {
    #[default]
    Closed,
    Open,
}

impl Plugin for InventoryUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InventoryState>();
        app.add_systems(OnEnter(GameState::Playing), button_init);
        app.add_systems(OnEnter(InventoryState::Open), spawn_inventory);
        app.add_systems(OnEnter(InventoryState::Closed), despawn_inventory);
    }
}

fn button_init(mut commands: Commands) {
    commands
        .spawn(button("Inventory"))
        .insert((
            DespawnOnExit(GameState::Playing),
            GlobalZIndex(LAYER_HUD),
            Tooltip {
                text: String::from("What might you find?"),
            },
            Node {
                position_type: PositionType::Absolute,

                bottom: Val::Px(50.0),
                right: Val::Px(50.0),
                ..default()
            },
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

fn despawn_inventory(mut commands: Commands, panel: Single<Entity, With<InventoryPanel>>) {
    commands
        .entity(*panel)
        .trigger(|p| DespawnPanel { entity: p });
}

fn spawn_inventory(
    mut commands: Commands,
    inventory: Single<&Inventory, With<Player>>,
    items: Query<(&ItemId, Option<&ItemStack>)>,
    item_store: Res<ItemStore>,
) {
    if inventory.0.is_empty() {
        return;
    }

    let rows: Vec<ItemRow> = inventory
        .0
        .iter()
        .filter_map(|&item_entity| {
            let (id, stack) = items.get(item_entity).ok()?;
            let store_item = item_store.0.get(&id.0)?;

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

    commands
        .spawn((
            Panel::default("Inventory".into()),
            InventoryPanel,
            Name::new("Inventory Panel"),
        ))
        .with_children(|p| {
            p.spawn(scroll_area(move |p| {
                p.spawn((
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
                ))
                .with_children(|grid| {
                    for item in rows {
                        let label = match item.count {
                            Some(count) => format!("{} ({})", item.name, count),
                            None => item.name.clone(),
                        };

                        grid.spawn((
                            Name::new(format!("Inventory Item '{}' (ID: {})", item.name, item.id)),
                            Node {
                                border: UiRect::all(Val::Px(1.)),
                                padding: UiRect::all(Val::Px(ITEM_PADDING)),
                                ..default()
                            },
                            BorderColor::all(ITEM_BORDER_COLOUR),
                            Tooltip::new(label),
                        ))
                        .with_children(|tile| {
                            tile.spawn((
                                Name::new(format!("Inventory Item Icon (ID: {})", item.id)),
                                ImageNode::new(item.icon),
                                Pickable::IGNORE,
                            ));

                            if let Some(count) = item.count.filter(|&c| c > 0) {
                                tile.spawn((
                                    Name::new(format!("Inventory Item Count (ID: {})", item.id)),
                                    Node {
                                        position_type: PositionType::Absolute,
                                        bottom: Val::Px(5.),
                                        right: Val::Px(5.),
                                        ..default()
                                    },
                                    Text::new(count.to_string()),
                                ));
                            }
                        });
                    }
                });
            }));
        });
}

struct ItemRow {
    id: String,
    name: String,
    description: String,
    count: Option<u32>,
    icon: Handle<Image>,
}
