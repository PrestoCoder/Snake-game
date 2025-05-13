mod snake;
mod pattern_generator;
mod level_state;

pub use snake::Snake;
pub use pattern_generator::{ObstaclePattern, get_level_pattern};
pub use level_state::LevelState;

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