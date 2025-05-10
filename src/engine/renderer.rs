use crate::{
    error::Result,
    game::GameState,
    types::Point,
};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::{Color, Print, SetForegroundColor, SetBackgroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    QueueableCommand,
};
use std::io::{stdout, Write, Stdout};

const BORDER_THICKNESS: u16 = 2;

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
            self.draw_point(point, Color::Green, Color::Black, "█")?;
        }
        
        // Draw food with a different character
        self.draw_point(game_state.food(), Color::Red, Color::Black, "●")?;
        
        // Draw score with a border
        self.draw_score(game_state.score())?;

        if game_state.is_game_over() {
            self.draw_game_over()?;
        }

        // Flush all queued changes
        self.stdout.flush()?;
        Ok(())
    }

    fn draw_borders(&mut self) -> Result<()> {
        // Set border color
        self.stdout
            .queue(SetForegroundColor(Color::Blue))?
            .queue(SetBackgroundColor(Color::Blue))?;

        // Draw horizontal borders (top and bottom) with thickness
        for y in 0..BORDER_THICKNESS {
            // Top border
            for x in 0..self.dimensions.0 {
                self.stdout
                    .queue(MoveTo(x, y))?
                    .queue(Print("█"))?;
            }
            // Bottom border
            for x in 0..self.dimensions.0 {
                self.stdout
                    .queue(MoveTo(x, self.dimensions.1 - 1 - y))?
                    .queue(Print("█"))?;
            }
        }

        // Draw vertical borders (left and right) with thickness
        for x in 0..BORDER_THICKNESS {
            // Left border
            for y in 0..self.dimensions.1 {
                self.stdout
                    .queue(MoveTo(x, y))?
                    .queue(Print("█"))?;
            }
            // Right border
            for y in 0..self.dimensions.1 {
                self.stdout
                    .queue(MoveTo(self.dimensions.0 - 1 - x, y))?
                    .queue(Print("█"))?;
            }
        }

        // Reset colors
        self.stdout
            .queue(SetForegroundColor(Color::Reset))?
            .queue(SetBackgroundColor(Color::Reset))?;

        Ok(())
    }

    fn draw_point(&mut self, point: &Point, fg_color: Color, bg_color: Color, symbol: &str) -> Result<()> {
        self.stdout
            .queue(MoveTo(point.x, point.y))?
            .queue(SetForegroundColor(fg_color))?
            .queue(SetBackgroundColor(bg_color))?
            .queue(Print(symbol))?;
        Ok(())
    }

    fn draw_score(&mut self, score: u32) -> Result<()> {
        let score_text = format!(" Score: {} ", score);
        let x = 2;
        let y = self.dimensions.1;

        // Draw score box
        self.stdout
            .queue(MoveTo(x - 1, y))?
            .queue(SetForegroundColor(Color::White))?
            .queue(SetBackgroundColor(Color::DarkBlue))?;

        // Draw score with background
        for (i, c) in score_text.chars().enumerate() {
            self.stdout
                .queue(MoveTo(x + i as u16, y))?
                .queue(Print(c))?;
        }

        // Reset colors
        self.stdout
            .queue(SetForegroundColor(Color::Reset))?
            .queue(SetBackgroundColor(Color::Reset))?;

        Ok(())
    }

    fn draw_game_over(&mut self) -> Result<()> {
        let game_over_text = " GAME OVER! Press any key to exit ";
        let x = (self.dimensions.0 - game_over_text.len() as u16) / 2;
        let y = self.dimensions.1 / 2;

        // Draw game over box with background
        self.stdout
            .queue(MoveTo(x, y))?
            .queue(SetForegroundColor(Color::White))?
            .queue(SetBackgroundColor(Color::Red))?
            .queue(Print(game_over_text))?;

        // Reset colors
        self.stdout
            .queue(SetForegroundColor(Color::Reset))?
            .queue(SetBackgroundColor(Color::Reset))?;

        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}