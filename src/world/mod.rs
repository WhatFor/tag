use bevy::prelude::*;

use crate::assets::AreaLoaderPlugin;
use crate::components::Health;
use crate::player::components::{Inventory, Player};
use crate::state::GameState;

pub mod components;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_game);
        app.add_plugins(AreaLoaderPlugin);
    }
}

fn setup_game(mut commands: Commands) {
    let player_entity: Entity = commands
        .spawn((
            Player,
            //NameComponent(None),
            Health(100),
            Inventory(vec![]),
            //CurrentArea(None),
        ))
        .id();
}
