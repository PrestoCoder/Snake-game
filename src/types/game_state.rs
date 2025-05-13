use super::GameEndReason;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Playing,
    LevelTransition,
    GameOver(GameEndReason),
}