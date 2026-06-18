use bevy::prelude::*;

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Pause(pub bool);

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Initialising,
    MainMenu,
    Introduction,
    Playing,
    GameOver,
}

#[derive(SubStates, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
#[source(GameState = GameState::Playing)]
pub enum PlayState {
    #[default]
    Exploring,
    InCombat,
}

#[derive(SubStates, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
#[source(PlayState = PlayState::Exploring)]
pub enum ExploringState {
    #[default]
    PresentingContent,
    AwaitingContinue,
    AwaitingChoice,
    AwaitingGameOver,
}
