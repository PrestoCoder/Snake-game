use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use std::time::Duration;
use crate::{types::Direction, error::Result};

pub struct InputHandler;

impl InputHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn get_input(&self) -> Result<Option<Direction>> {
        if poll(Duration::from_millis(1))? {
            if let Event::Key(key_event) = read()? {
                if key_event.kind == KeyEventKind::Press {
                    return Ok(match key_event.code {
                        KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => Some(Direction::Up),
                        KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => Some(Direction::Down),
                        KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => Some(Direction::Left),
                        KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => Some(Direction::Right),
                        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                            // Set game over instead of directly exiting
                            None
                        }
                        _ => None,
                    });
                }
            }
        }
        Ok(None)
    }
}