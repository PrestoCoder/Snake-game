use crate::{
    error::Result,
    game::GameState,
    types::Point,
};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    QueueableCommand,
};
use std::io::{stdout, Write, Stdout};

pub struct Renderer {
    dimensions: (u16, u16),
    stdout: Stdout,
}

impl Renderer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            dimensions: (width, height),
            stdout: stdout(),
        }
    }

    pub fn init(&mut self) -> Result<()> {
        execute!(
            self.stdout,
            EnterAlternateScreen,
            Hide,
        )?;
        Ok(())
    }

    pub fn cleanup(&mut self) -> Result<()> {
        execute!(
            self.stdout,
            Show,
            LeaveAlternateScreen,
        )?;
        Ok(())
    }

    pub fn render(&mut self, game_state: &GameState) -> Result<()> {
        // Clear screen
        self.stdout.queue(Clear(ClearType::All))?;
        
        // Draw borders
        self.draw_borders()?;
        
        // Draw snake
        for point in game_state.snake() {
            self.draw_point(point, Color::Green)?;
        }
        
        // Draw food
        self.draw_point(game_state.food(), Color::Red)?;
        
        // Draw score
        self.stdout
            .queue(MoveTo(0, self.dimensions.1))?
            .queue(SetForegroundColor(Color::White))?
            .queue(Print(format!("Score: {}", game_state.score())))?;

        if game_state.is_game_over() {
            self.draw_game_over()?;
        }

        // Flush all queued changes
        self.stdout.flush()?;
        Ok(())
    }

    fn draw_borders(&mut self) -> Result<()> {
        // Draw horizontal borders
        for x in 0..self.dimensions.0 {
            self.draw_point(&Point::new(x, 0), Color::White)?;
            self.draw_point(&Point::new(x, self.dimensions.1 - 1), Color::White)?;
        }

        // Draw vertical borders
        for y in 0..self.dimensions.1 {
            self.draw_point(&Point::new(0, y), Color::White)?;
            self.draw_point(&Point::new(self.dimensions.0 - 1, y), Color::White)?;
        }

        Ok(())
    }

    fn draw_point(&mut self, point: &Point, color: Color) -> Result<()> {
        self.stdout
            .queue(MoveTo(point.x, point.y))?
            .queue(SetForegroundColor(color))?
            .queue(Print("█"))?;
        Ok(())
    }

    fn draw_game_over(&mut self) -> Result<()> {
        let game_over_text = "Game Over! Press any key to exit.";
        let x = (self.dimensions.0 - game_over_text.len() as u16) / 2;
        let y = self.dimensions.1 / 2;

        self.stdout
            .queue(MoveTo(x, y))?
            .queue(SetForegroundColor(Color::Red))?
            .queue(Print(game_over_text))?;
        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}