use crate::prelude::*;
use bevy::prelude::*;

use crate::ui::widgets::button::button;

pub mod components;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(ExploringState::AwaitingContinue),
            show_continue_prompt.in_set(PausableSystems),
        );

        app.add_systems(
            Update,
            wait_for_continue_input
                .run_if(in_state(ExploringState::AwaitingContinue))
                .in_set(PausableSystems),
        );
    }
}

fn show_continue_prompt(mut commands: Commands) {
    commands
        .spawn((
            DespawnOnExit(ExploringState::AwaitingContinue),
            GlobalZIndex(10),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(250.0),
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|p| {
            p.spawn(button("Continue"))
                .observe(|_: On<Pointer<Click>>, mut commands: Commands| {
                    commands.trigger(PlayerContinued);
                });
        });
}

fn wait_for_continue_input(mut commands: Commands, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        commands.trigger(PlayerContinued);
    }
}
