use crate::prelude::*;
use bevy::prelude::*;

use crate::persistence::SAVE_FILE_KEY;
use crate::persistence::events::SaveDeleted;
use crate::persistence::resources::SaveBackend;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), init);
        app.add_systems(OnEnter(GameState::GameOver), reset_to_menu);
    }
}

fn init(mut commands: Commands, save_store: Res<SaveBackend>) {
    commands
        .spawn((
            Node {
                // center button
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.),
                ..default()
            },
            GlobalZIndex(LAYER_HUD),
            Name::new("Main Menu Container"),
            DespawnOnExit(GameState::MainMenu),
        ))
        .with_children(|p| {
            if let Ok(save_file_exists) = save_store.exists(SAVE_FILE_KEY) {
                if save_file_exists {
                    p.spawn(button("Continue")).observe(
                        |_: On<Pointer<Click>>,
                         mut commands: Commands,
                         mut next_state: ResMut<NextState<GameState>>| {
                            commands.trigger(PlaySoundtrack);
                            next_state.set(GameState::Playing);
                        },
                    );
                }
            }

            p.spawn(button("New game")).observe(
                |_: On<Pointer<Click>>,
                 mut commands: Commands,
                 mut next_state: ResMut<NextState<GameState>>| {
                    commands.trigger(SaveDeleted);
                    commands.trigger(PlaySoundtrack);
                    next_state.set(GameState::Introduction);
                },
            );

            p.spawn(button("Settings")).observe(
                |_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                    next_state.set(GameState::SettingsMenu);
                },
            );
        });
}

fn reset_to_menu(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_exploring_state: ResMut<NextState<ExploringState>>,
    save_store: Res<SaveBackend>,
) -> Result {
    next_game_state.set(GameState::MainMenu);
    next_exploring_state.set(ExploringState::PresentingContent);
    save_store.clear(SAVE_FILE_KEY)?;

    Ok(())
}
