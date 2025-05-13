pub mod levels;
mod snake;

pub use snake::Snake;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Playing,
    LevelTransition,
    GameOver(GameEndReason),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameEndReason {
    Collision,
    Victory,
}