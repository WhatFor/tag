use bevy::prelude::*;

use crate::state::ExploringState;

#[derive(Component)]
struct ContinuePrompt;

pub struct ContinuePromptUIPlugin;

impl Plugin for ContinuePromptUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(ExploringState::AwaitingContinue),
            show_continue_prompt,
        );
        app.add_systems(
            OnExit(ExploringState::AwaitingContinue),
            hide_continue_prompt,
        );
    }
}

fn show_continue_prompt(mut commands: Commands, fonts: Res<FontAssets>) {
    commands.spawn((
        ContinuePrompt,
        Text::new("Press Space to continue..."),
        fonts.narration_font.clone(),
        fonts.narration_color,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(5.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
    ));
}

fn hide_continue_prompt(mut commands: Commands, prompt: Single<Entity, With<ContinuePrompt>>) {
    commands.entity(*prompt).despawn();
}
