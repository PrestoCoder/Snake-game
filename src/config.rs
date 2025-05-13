// config.rs

// Game dimensions and timing
pub const WIDTH: u16 = 50;
pub const HEIGHT: u16 = 25;
pub const BORDER_THICKNESS: u16 = 2;

// Speed settings
pub const BASE_TICK_RATE: u64 = 200;
pub const SPEED_DECREASE_PER_LEVEL: u64 = 10;
pub const MIN_SPEED: u64 = 50;
pub const BASE_SPEED_LEVEL: u32 = 1;

// Level settings
pub const STARTING_LEVEL: u32 = 1;
pub const MAX_LEVELS: u32 = 3;
pub const SCORE_PER_LEVEL: u32 = 5;

// Obstacle settings
pub const BASE_OBSTACLES: u32 = 4;
pub const OBSTACLES_PER_LEVEL: u32 = 2;
pub const OBSTACLE_SIZES: [u16; 2] = [1, 2];

// For code that expects a Config struct
pub struct Config {
    pub width: u16,
    pub height: u16,
    pub tick_rate_ms: u64,
    pub starting_level: u32,
    pub max_levels: u32,
    pub score_per_level: u32,
    pub base_obstacles: u32,
    pub obstacles_per_level: u32,
    pub obstacle_sizes: Vec<u16>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            width: WIDTH,
            height: HEIGHT,
            tick_rate_ms: BASE_TICK_RATE,
            starting_level: STARTING_LEVEL,
            max_levels: MAX_LEVELS,
            score_per_level: SCORE_PER_LEVEL,
            base_obstacles: BASE_OBSTACLES,
            obstacles_per_level: OBSTACLES_PER_LEVEL,
            obstacle_sizes: OBSTACLE_SIZES.to_vec(),
        }
    }
}