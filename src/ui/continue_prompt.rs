use bevy::prelude::*;

use crate::{sets::PlayingSet, state::ExploringState, ui::FontAssets};

#[derive(Component)]
struct ContinuePrompt;

pub struct ContinuePromptUIPlugin;

impl Plugin for ContinuePromptUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(ExploringState::AwaitingContinue),
            show_continue_prompt.in_set(PlayingSet::InitialiseUI),
        );
        app.add_systems(
            OnExit(ExploringState::AwaitingContinue),
            hide_continue_prompt.in_set(PlayingSet::InitialiseUI),
        );
    }
}

fn show_continue_prompt(mut commands: Commands, fonts: Res<FontAssets>) {
    commands.spawn((
        ContinuePrompt,
        Name::new("Continue Prompt"),
        Text::new("~ Click or Spacebar to continue ~"),
        fonts.ui_font.clone(),
        fonts.ui_color,
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(2.0),
            width: Val::Percent(100.0),
            ..default()
        },
    ));
}

fn hide_continue_prompt(mut commands: Commands, prompt: Single<Entity, With<ContinuePrompt>>) {
    commands.entity(*prompt).despawn();
}
