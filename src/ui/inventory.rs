use bevy::prelude::*;

use crate::{state::GameState, ui::widgets::button::button};

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
