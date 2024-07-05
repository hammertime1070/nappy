use bevy::prelude::*;

// #[derive(States)]
// pub enum AppState {
//     #[default]
//     LoadingAssets,
//     InGame,
//     GameOver,
// }

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Uninitialized,
    Initializing,
    PlayerTurn,
    EnemyTurn,
}



