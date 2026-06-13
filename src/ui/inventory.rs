use crate::prelude::*;
use bevy::prelude::*;

use crate::ui::widgets::panel::DespawnPanel;

pub struct InventoryUIPlugin;

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
    items: Query<(&DisplayName, &Description, Option<&ItemStack>)>,
) {
    if inventory.0.is_empty() {
        return;
    }

    let rows: Vec<String> = inventory
        .0
        .iter()
        .filter_map(|&item_entity| {
            let (name, _desc, stack) = items.get(item_entity).ok()?;
            Some(match stack {
                Some(ItemStack(count)) => format!("{} ({})", name.0, count),
                None => name.0.clone(),
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
                for label in rows {
                    p.spawn(Text::new(label));
                }
            }));
        });
}
