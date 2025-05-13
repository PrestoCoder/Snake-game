use serde::Deserialize;
use crate::utils::Result;

#[derive(Debug, Deserialize)]
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
    pub fn load() -> Result<Self> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name("config/default"))
            .build()?;

        Ok(settings.try_deserialize()?)
    }
}