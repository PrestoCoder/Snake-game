mod level1;
mod level2;
mod level3;

pub struct ObstaclePattern {
    pub positions: Vec<(u16, u16)>,
    pub sizes: Vec<(u16, u16)>,
}

pub fn get_level_pattern(level: u32, width: u16, height: u16) -> ObstaclePattern {
    match level {
        1 => level1::get_pattern(width, height),
        2 => level2::get_pattern(width, height),
        3 => level3::get_pattern(width, height),
        _ => level3::get_pattern(width, height), // Default to level 3 pattern
    }
}