use crate::prelude::*;
use bevy::prelude::*;

use crate::persistence::events::{LoadCheckpoint, SaveDeleted};
use crate::player::components::Hardcore;

pub struct DeadUIPlugin;

impl Plugin for DeadUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Dead), show_game_over);
    }
}

fn show_game_over(
    mut commands: Commands,
    last_death: Res<LastDeath>,
    hardcore: Single<&Hardcore, With<Player>>,
) {
    commands
        .spawn((
            DespawnOnExit(GameState::Dead),
            GlobalZIndex(LAYER_MENU),
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor::from(Color::srgb(0., 0., 0.)),
        ))
        .with_children(|p| {
            p.spawn(Node {
                width: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.),
                ..default()
            })
            .with_children(|p| {
                p.spawn(Text::new("Game over."));

                match &last_death.0 {
                    DeathReason::NoHealth => {
                        p.spawn(Text::new("You ran out of health."));
                    }
                    DeathReason::Ending(reason) => {
                        p.spawn(Text::new(reason));
                    }
                }

                if hardcore.0 == false {
                    p.spawn(button("Back to Checkpoint")).observe(
                        |_: On<Pointer<Click>>,
                         mut commands: Commands,
                         last_checkpoint: Single<&LastCheckpointArea, With<Player>>,
                         areas: Query<&AreaId, With<Area>>,
                         mut next_game_state: ResMut<NextState<GameState>>|
                         -> Result {
                            // Move back to playing
                            next_game_state.set(GameState::Playing);

                            // Trigger load of checkpoint
                            let area_id = areas.get(last_checkpoint.0)?.0.clone();
                            commands.trigger(LoadCheckpoint { area_id: area_id });

                            Ok(())
                        },
                    );
                }

                p.spawn(button("Main menu")).observe(
                    |_: On<Pointer<Click>>,
                     mut commands: Commands,
                     mut next_game_state: ResMut<NextState<GameState>>|
                     -> Result {
                        // Clear save
                        commands.trigger(SaveDeleted);

                        // Return to menu
                        commands.trigger(StopSoundtrack);
                        next_game_state.set(GameState::MainMenu);

                        Ok(())
                    },
                );
            });
        });
}
