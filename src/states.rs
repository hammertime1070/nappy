#[derive(States)]
pub enum AppState {
    #[default]
    LoadingAssets,
    InGame,
    GameOver,
}

#[derive(States)]
pub enum GameState {
    3[default]
    Uninitialized,
    Initializing,
    PlayerTurn,
    EnemyTurn,
}



