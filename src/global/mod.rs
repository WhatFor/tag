use crate::prelude::*;
use bevy::prelude::*;

/// A system set for systems that shouldn't run while the game is paused
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PausableSystems;

pub struct GlobalPlugin;

impl Plugin for GlobalPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.init_resource::<AudioSettings>();
        app.add_sub_state::<PlayState>();
        app.add_sub_state::<ExploringState>();

        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));

        app.add_systems(Startup, setup_global);

        // Ensure the World and UI is spawned before the Player
        // to allow setting the CurrentArea component on the Player (which
        // holds a ref to an Area Entity).
        app.configure_sets(
            OnEnter(GameState::Playing),
            (
                PlayingSet::InitialiseUI,
                PlayingSet::SpawnAreas,
                PlayingSet::SpawnPlayer,
            )
                .chain(),
        );
    }
}

fn setup_global(mut commands: Commands) {
    commands.spawn((Camera2d, Name::new("2D Camera")));
}
