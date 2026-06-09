use crate::prelude::*;
use bevy::prelude::*;

use crate::game::progression::ProgressionPlugin;
use bevy::input::common_conditions::input_just_pressed;

pub mod events;

mod progression;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProgressionPlugin);

        app.add_systems(
            Update,
            (pause, spawn_pause_overlay).run_if(
                in_state(Pause(false))
                    .and(in_state(GameState::Playing))
                    .and(input_just_pressed(KeyCode::Escape)),
            ),
        );

        app.add_systems(
            Update,
            unpause.run_if(
                in_state(Pause(true))
                    .and(in_state(GameState::Playing))
                    .and(input_just_pressed(KeyCode::Escape)),
            ),
        );
    }
}

fn unpause(mut next_pause_state: ResMut<NextState<Pause>>) {
    next_pause_state.set(Pause(false));
}

fn pause(mut next_pause_state: ResMut<NextState<Pause>>) {
    next_pause_state.set(Pause(true));
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        Name::new("Pause Overlay"),
        GlobalZIndex(1),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 1.0)),
        DespawnOnExit(Pause(true)),
        Pickable::IGNORE,
    ));
}
