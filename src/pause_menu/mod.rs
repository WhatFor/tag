use crate::persistence::events::LoadCheckpoint;
use crate::prelude::*;
use bevy::prelude::*;

use crate::ui::inventory::InventoryState;
use crate::ui::widgets::button::button;
use bevy::input::common_conditions::input_just_pressed;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Pause(true)), init);

        app.add_systems(
            Update,
            (pause, spawn_pause_overlay).run_if(
                in_state(Pause(false))
                    .and(in_state(GameState::Playing))
                    .and(in_state(InventoryState::Closed))
                    .and(input_just_pressed(KeyCode::Escape)),
            ),
        );

        app.add_systems(
            Update,
            unpause.run_if(
                in_state(Pause(true))
                    .and(in_state(GameState::Playing))
                    .and(in_state(InventoryState::Closed))
                    .and(input_just_pressed(KeyCode::Escape)),
            ),
        );
    }
}

fn init(mut commands: Commands) {
    commands
        .spawn((
            GlobalZIndex(LAYER_PAUSE),
            DespawnOnExit(Pause(true)),
            Name::new("Pause Menu Container"),
            Node {
                // center button
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.),
                ..default()
            },
        ))
        .with_children(|p| {
            p.spawn(button("Main Menu")).observe(
                |_: On<Pointer<Click>>,
                 mut commands: Commands,
                 mut next_game_state: ResMut<NextState<GameState>>,
                 mut next_pause_state: ResMut<NextState<Pause>>| {
                    commands.trigger(StopSoundtrack);
                    next_pause_state.set(Pause(false));
                    next_game_state.set(GameState::MainMenu);
                },
            );

            p.spawn(button("Reset to Checkpoint")).observe(
                |_: On<Pointer<Click>>,
                 mut commands: Commands,
                 last_checkpoint: Single<&LastCheckpointArea, With<Player>>,
                 areas: Query<&AreaId, With<Area>>,
                 mut next_pause_state: ResMut<NextState<Pause>>|
                 -> Result {
                    let area_id = areas.get(last_checkpoint.0)?.0.clone();
                    commands.trigger(LoadCheckpoint { area_id: area_id });

                    next_pause_state.set(Pause(false));

                    Ok(())
                },
            );

            // p.spawn(button("Settings")).observe(
            //     |_: On<Pointer<Click>>, mut next_game_state: ResMut<NextState<GameState>>| {
            //         next_game_state.set(GameState::SettingsMenu);
            //     },
            // );
        });
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
        GlobalZIndex(LAYER_PAUSE - 1),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 1.0)),
        DespawnOnExit(Pause(true)),
        Pickable::IGNORE,
    ));
}
