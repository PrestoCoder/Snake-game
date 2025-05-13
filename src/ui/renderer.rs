use crate::{
    utils::Result,
    core::GameState,
    entities::{Point, Obstacle},
    gameplay::GameEndReason,
};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::{Color, Print, SetForegroundColor, SetBackgroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    QueueableCommand,
};
use std::io::{stdout, Write, Stdout};
use crate::utils::constants::BORDER_THICKNESS;

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
        
        match game_state.game_state() {
            gameplay::GameState::Playing => {
                self.draw_borders()?;
                self.draw_obstacles(game_state.obstacles())?;
                
                for point in game_state.snake() {
                    self.draw_point(point, Color::Green, Color::Reset, "█")?;
                }
                
                self.draw_point(game_state.food(), Color::Red, Color::Reset, "●")?;
                self.draw_score(game_state)?;
            }
            gameplay::GameState::LevelTransition => {
                self.draw_level_transition(game_state)?;
            }
            gameplay::GameState::GameOver(reason) => {
                self.draw_game_over(game_state, reason)?;
            }
        }

        self.stdout.flush()?;
        Ok(())
    }

    fn draw_borders(&mut self) -> Result<()> {
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

        self.stdout
            .queue(SetForegroundColor(Color::Reset))?
            .queue(SetBackgroundColor(Color::Reset))?;

        Ok(())
    }

    fn draw_obstacles(&mut self, obstacles: &[Obstacle]) -> Result<()> {
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

    fn draw_level_transition(&mut self, game_state: &GameState) -> Result<()> {
        let message = game_state.transition_message();
        let lines: Vec<&str> = message.split('\n').collect();
        
        let y_start = (self.dimensions.1 / 2) - (lines.len() as u16 / 2);

        // Draw a box around the transition message
        let max_width = lines.iter().map(|line| line.len()).max().unwrap_or(0) as u16;
        let padding = 2;
        let box_width = max_width + (padding * 2);
        let box_height = lines.len() as u16 + (padding * 2);
        let box_x = (self.dimensions.0 - box_width) / 2;
        let box_y = y_start - padding;

        // Draw box background
        for y in 0..box_height {
            for x in 0..box_width {
                self.stdout
                    .queue(MoveTo(box_x + x, box_y + y))?
                    .queue(SetBackgroundColor(Color::DarkBlue))?
                    .queue(Print(" "))?;
            }
        }

        // Draw message
        for (i, line) in lines.iter().enumerate() {
            let x = (self.dimensions.0 - line.len() as u16) / 2;
            let y = y_start + i as u16;

            self.stdout
                .queue(MoveTo(x, y))?
                .queue(SetForegroundColor(Color::White))?
                .queue(Print(line))?;
        }

        self.stdout.queue(SetBackgroundColor(Color::Reset))?;
        Ok(())
    }

    fn draw_game_over(&mut self, game_state: &GameState, reason: GameEndReason) -> Result<()> {
        let message = match reason {
            GameEndReason::Victory => format!(
                "VICTORY!\nFinal Score: {}\nAll {} Levels Complete!\nPress 'q' to quit",
                game_state.score(),
                game_state.max_levels()
            ),
            GameEndReason::Collision => format!(
                "GAME OVER!\nFinal Score: {}\nLevel {} of {}\nPress 'q' to quit",
                game_state.score(),
                game_state.current_level(),
                game_state.max_levels()
            ),
        };

        let lines: Vec<&str> = message.split('\n').collect();
        let y_start = (self.dimensions.1 / 2) - (lines.len() as u16 / 2);

        let bg_color = match reason {
            GameEndReason::Victory => Color::Green,
            GameEndReason::Collision => Color::Red,
        };

        // Draw a box around the game over message
        let max_width = lines.iter().map(|line| line.len()).max().unwrap_or(0) as u16;
        let padding = 2;
        let box_width = max_width + (padding * 2);
        let box_height = lines.len() as u16 + (padding * 2);
        let box_x = (self.dimensions.0 - box_width) / 2;
        let box_y = y_start - padding;

        // Draw box background
        for y in 0..box_height {
            for x in 0..box_width {
                self.stdout
                    .queue(MoveTo(box_x + x, box_y + y))?
                    .queue(SetBackgroundColor(bg_color))?
                    .queue(Print(" "))?;
            }
        }

        // Draw message
        for (i, line) in lines.iter().enumerate() {
            let x = (self.dimensions.0 - line.len() as u16) / 2;
            let y = y_start + i as u16;

            self.stdout
                .queue(MoveTo(x, y))?
                .queue(SetForegroundColor(Color::White))?
                .queue(Print(line))?;
        }

        self.stdout.queue(SetBackgroundColor(Color::Reset))?;
        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}