use bevy::prelude::*;

use crate::{state::GameState, ui::widgets::button::button};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), init);
    }
}

fn init(mut commands: Commands) {
    commands
        .spawn((
            Node {
                // center button
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Name::new("Main Menu Container"),
            DespawnOnExit(GameState::MainMenu),
        ))
        .with_children(|p| {
            p.spawn(button("Play")).observe(
                |_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                    next_state.set(GameState::Playing);
                },
            );
        });
}
