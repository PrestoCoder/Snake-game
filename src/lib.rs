pub mod core;
pub mod gameplay;
pub mod ui;
pub mod entities;
pub mod utils;

pub mod config;
pub use config::*;  // Export everything from config
pub use utils::{Result, GameError};


