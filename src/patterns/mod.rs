mod types;
mod levels;

pub use types::ObstaclePattern;
use levels::*;

pub fn get_level_pattern(level: u32, width: u16, height: u16) -> ObstaclePattern {
    let center_x = width / 2;
    let center_y = height / 2;

    match level {
        1 => level_one_pattern(center_x, center_y),
        2 => level_two_pattern(width, height),
        3 => level_three_pattern(center_x, center_y),
        _ => level_three_pattern(center_x, center_y), // Default to level 3 pattern
    }
}