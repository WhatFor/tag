use crate::{prelude::*, ui::widgets::panel::DespawnPanel};
use bevy::prelude::*;

use crate::ui::widgets::button::button;

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

fn spawn_inventory(mut commands: Commands) {
    commands
        .spawn((
            Panel::default("Inventory".into()),
            InventoryPanel,
            Name::new("Inventory Panel"),
        ))
        .with_children(|p| {
            p.spawn(Text::new("Placeholder"));
        });
}
