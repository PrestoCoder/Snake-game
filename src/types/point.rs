use serde::{Deserialize, Serialize};
use super::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn translate(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Self::new(self.x, self.y.wrapping_sub(1)),
            Direction::Down => Self::new(self.x, self.y.wrapping_add(1)),
            Direction::Left => Self::new(self.x.wrapping_sub(1), self.y),
            Direction::Right => Self::new(self.x.wrapping_add(1), self.y),
        }
    }
}
