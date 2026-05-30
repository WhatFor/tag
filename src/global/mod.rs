use bevy::log::*;
use bevy::prelude::*;

use crate::sets::PlayingSet;
use crate::state::ExploringState;
use crate::state::{GameState, PlayState};

#[derive(Resource, Default)]
pub struct PendingAssets(pub Vec<UntypedHandle>);

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PausableSystems;

pub struct GlobalPlugin;

impl Plugin for GlobalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PendingAssets>();
        app.init_state::<GameState>();
        app.add_sub_state::<PlayState>();
        app.add_sub_state::<ExploringState>();

        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));

        app.add_systems(Startup, setup_global);

        app.add_systems(
            Update,
            check_all_asset_progress.run_if(in_state(GameState::Initialising)),
        );

        // Ensure the World is spawned before the Player
        // to allow setting the CurrentArea component on the Player (which
        // holds a ref to an Area Entity).
        app.configure_sets(
            OnEnter(GameState::Playing),
            PlayingSet::SpawnWorld.before(PlayingSet::SpawnPlayer),
        );
    }
}

fn check_all_asset_progress(
    pending: Res<PendingAssets>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !pending.0.is_empty()
        && pending
            .0
            .iter()
            .all(|h| asset_server.is_loaded_with_dependencies(h))
    {
        info!("All assets loaded! Moving to menu...");
        next_state.set(GameState::MainMenu);
    }
}

fn setup_global(mut commands: Commands) {
    commands.spawn(Camera2d);
}
