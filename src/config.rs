use serde::Deserialize;
use crate::error::Result;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub width: u16,
    pub height: u16,
    pub tick_rate_ms: u64,
}

impl Config {
    pub fn load() -> Result<Self> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name("config/default"))
            .build()?;

        Ok(settings.try_deserialize()?)
    }
}
