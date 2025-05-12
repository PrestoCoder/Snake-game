#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameEndReason {
    Collision,
    Victory,
}