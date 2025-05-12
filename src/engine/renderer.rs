use crate::{
    error::Result,
    game::GameState,
    types::{Point, Obstacle, GameEndReason},
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
        self.stdout.queue(Clear(ClearType::All))?;
        
        self.draw_borders()?;
        self.draw_obstacles(game_state.obstacles())?;
        
        for point in game_state.snake() {
            self.draw_point(point, Color::Green, Color::Reset, "█")?;
        }
        
        self.draw_point(game_state.food(), Color::Red, Color::Reset, "●")?;
        
        self.draw_score(game_state)?;

        if game_state.is_game_over() {
            self.draw_game_over(game_state)?;
        }

        self.stdout.flush()?;
        Ok(())
    }

    fn draw_borders(&mut self) -> Result<()> {
        // Set border color
        self.stdout
            .queue(SetForegroundColor(Color::Blue))?
            .queue(SetBackgroundColor(Color::Blue))?;

        // Draw horizontal borders
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

        // Draw vertical borders
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

    fn draw_obstacles(&mut self, obstacles: &Vec<Obstacle>) -> Result<()> {
        for obstacle in obstacles {
            for point in &obstacle.blocks {
                self.draw_point(point, Color::DarkGrey, Color::DarkGrey, "█")?;
            }
        }
        Ok(())
    }

    fn draw_point(&mut self, point: &Point, fg_color: Color, bg_color: Color, symbol: &str) -> Result<()> {
        self.stdout
            .queue(MoveTo(point.x, point.y))?
            .queue(SetForegroundColor(fg_color))?
            .queue(SetBackgroundColor(bg_color))?
            .queue(Print(symbol))?
            .queue(SetBackgroundColor(Color::Reset))?;
        Ok(())
    }

    fn draw_score(&mut self, game_state: &GameState) -> Result<()> {
        let next_score = game_state.score_needed_for_next()
            .map(|s| format!("/{}", s))
            .unwrap_or_else(|| "".to_string());

        let stats_text = format!(
            " Level: {}/{} | Score: {}{} | Speed: {} ", 
            game_state.current_level(),
            game_state.max_levels(),
            game_state.score(),
            next_score,
            game_state.speed_level(),
        );
        
        let x = 2;
        let y = self.dimensions.1;

        self.stdout
            .queue(MoveTo(x, y))?
            .queue(SetForegroundColor(Color::White))?
            .queue(SetBackgroundColor(Color::DarkBlue))?
            .queue(Print(&stats_text))?
            .queue(SetBackgroundColor(Color::Reset))?;

        Ok(())
    }

    fn draw_game_over(&mut self, game_state: &GameState) -> Result<()> {
        let message = match game_state.end_reason() {
            Some(GameEndReason::Victory) => format!(" VICTORY! Final Score: {} ", game_state.score()),
            _ => " GAME OVER! Press any key to exit ".to_string(),
        };

        let x = (self.dimensions.0 - message.len() as u16) / 2;
        let y = self.dimensions.1 / 2;

        // Use green for victory, red for game over
        let bg_color = match game_state.end_reason() {
            Some(GameEndReason::Victory) => Color::Green,
            _ => Color::Red,
        };

        self.stdout
            .queue(MoveTo(x, y))?
            .queue(SetForegroundColor(Color::White))?
            .queue(SetBackgroundColor(bg_color))?
            .queue(Print(&message))?
            .queue(SetBackgroundColor(Color::Reset))?;

        // Show target score for current level
        if let Some(next_score) = game_state.score_needed_for_next() {
            let target_text = format!(" Target Score: {} ", next_score);
            let tx = (self.dimensions.0 - target_text.len() as u16) / 2;
            
            self.stdout
                .queue(MoveTo(tx, y - 2))?
                .queue(SetForegroundColor(Color::White))?
                .queue(SetBackgroundColor(Color::DarkBlue))?
                .queue(Print(&target_text))?
                .queue(SetBackgroundColor(Color::Reset))?;
        }

        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}