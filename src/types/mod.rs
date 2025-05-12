mod direction;
mod point;
mod obstacle;

pub use direction::Direction;
pub use point::Point;
pub use obstacle::Obstacle;

// Re-export types that might be needed elsewhere
pub use direction::Direction::{Up, Down, Left, Right};