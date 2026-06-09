use crate::prelude::*;
use bevy::prelude::*;

use crate::ui::widgets::button::button;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Pause(true)), init);
    }
}

fn init(mut commands: Commands) {
    commands
        .spawn((
            GlobalZIndex(2),
            DespawnOnExit(Pause(true)),
            Name::new("Pause Menu Container"),
            Node {
                // center button
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|p| {
            p.spawn(button("Main Menu")).observe(
                |_: On<Pointer<Click>>,
                 mut next_game_state: ResMut<NextState<GameState>>,
                 mut next_pause_state: ResMut<NextState<Pause>>| {
                    next_pause_state.set(Pause(false));
                    next_game_state.set(GameState::MainMenu);
                },
            );
        });
}
