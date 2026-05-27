use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Initialising,
    MainMenu,
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
