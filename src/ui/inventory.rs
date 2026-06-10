use crate::prelude::*;
use bevy::prelude::*;

use crate::ui::widgets::button::button;

pub struct InventoryUIPlugin;

impl Plugin for InventoryUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), button_init);
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
        .observe(|_: On<Pointer<Click>>| {
            // TODO: Open inventory
        });
}
