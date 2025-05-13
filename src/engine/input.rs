use crossterm::event::{poll, read, Event, KeyCode};
use std::time::Duration;
use crate::error::Result;

pub struct InputHandler;

impl InputHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn get_input(&self) -> Result<Option<KeyCode>> {
        if poll(Duration::from_millis(0))? {
            if let Event::Key(key_event) = read()? {
                return Ok(Some(key_event.code));
            }
        }
        Ok(None)
    }
}