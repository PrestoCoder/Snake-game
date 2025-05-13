use crate::{
    utils::Result,
    core::GameState,
    entities::{Point, Obstacle},
    gameplay::{GameState as GameStateEnum, GameEndReason},
    config::BORDER_THICKNESS
};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::{Color, Print, SetForegroundColor, SetBackgroundColor},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
    QueueableCommand,
};
use std::io::{stdout, Write, Stdout};

pub struct Renderer {
    dimensions: (u16, u16),
    stdout: Stdout,
    previous_frame: Vec<Vec<(char, Color, Color)>>,
    current_frame: Vec<Vec<(char, Color, Color)>>,
}

impl Renderer {
    pub fn new(width: u16, height: u16) -> Self {
        let empty_frame = vec![vec![(' ', Color::Reset, Color::Reset); width as usize]; height as usize];
        Self {
            dimensions: (width, height),
            stdout: stdout(),
            previous_frame: empty_frame.clone(),
            current_frame: empty_frame,
        }
    }

    pub fn init(&mut self) -> Result<()> {
        enable_raw_mode()?;
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
        disable_raw_mode()?;
        Ok(())
    }

    pub fn render(&mut self, game_state: &GameState) -> Result<()> {
        // Clear current frame
        for row in self.current_frame.iter_mut() {
            for cell in row.iter_mut() {
                *cell = (' ', Color::Reset, Color::Reset);
            }
        }

        // Draw game elements to buffer
        match game_state.game_state() {
            GameStateEnum::Playing => {
                self.draw_to_buffer_borders()?;
                self.draw_to_buffer_obstacles(game_state.obstacles())?;
                
                for point in game_state.snake().body() {
                    self.set_buffer_cell(point, '█', Color::Green, Color::Reset)?;
                }
                
                self.set_buffer_cell(game_state.food().position(), '●', Color::Red, Color::Reset)?;
                self.draw_to_buffer_status(game_state)?;
            }
            GameStateEnum::LevelTransition => {
                self.draw_to_buffer_transition(game_state)?;
            }
            GameStateEnum::GameOver(reason) => {
                self.draw_to_buffer_game_over(game_state, reason)?;
            }
        }

        // Render only changed cells
        for y in 0..self.dimensions.1 as usize {
            for x in 0..self.dimensions.0 as usize {
                if self.current_frame[y][x] != self.previous_frame[y][x] {
                    let (ch, fg, bg) = self.current_frame[y][x];
                    self.stdout
                        .queue(MoveTo(x as u16, y as u16))?
                        .queue(SetForegroundColor(fg))?
                        .queue(SetBackgroundColor(bg))?
                        .queue(Print(ch))?;
                }
            }
        }

        // Flush changes
        self.stdout.flush()?;

        // Swap buffers
        std::mem::swap(&mut self.current_frame, &mut self.previous_frame);

        Ok(())
    }

    fn set_buffer_cell(&mut self, point: &Point, ch: char, fg: Color, bg: Color) -> Result<()> {
        if point.x < self.dimensions.0 && point.y < self.dimensions.1 {
            self.current_frame[point.y as usize][point.x as usize] = (ch, fg, bg);
        }
        Ok(())
    }

    fn draw_to_buffer_borders(&mut self) -> Result<()> {
        // Draw horizontal borders
        for y in 0..BORDER_THICKNESS {
            for x in 0..self.dimensions.0 {
                self.set_buffer_cell(&Point::new(x, y), '█', Color::Blue, Color::Blue)?;
                self.set_buffer_cell(
                    &Point::new(x, self.dimensions.1 - 1 - y),
                    '█',
                    Color::Blue,
                    Color::Blue,
                )?;
            }
        }

        // Draw vertical borders
        for x in 0..BORDER_THICKNESS {
            for y in 0..self.dimensions.1 {
                self.set_buffer_cell(&Point::new(x, y), '█', Color::Blue, Color::Blue)?;
                self.set_buffer_cell(
                    &Point::new(self.dimensions.0 - 1 - x, y),
                    '█',
                    Color::Blue,
                    Color::Blue,
                )?;
            }
        }

        Ok(())
    }

    fn draw_to_buffer_obstacles(&mut self, obstacles: &[Obstacle]) -> Result<()> {
        for obstacle in obstacles {
            for point in &obstacle.blocks {
                self.set_buffer_cell(point, '█', Color::DarkGrey, Color::DarkGrey)?;
            }
        }
        Ok(())
    }

    fn draw_to_buffer_status(&mut self, game_state: &GameState) -> Result<()> {
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

        for (i, ch) in stats_text.chars().enumerate() {
            self.set_buffer_cell(
                &Point::new(2 + i as u16, self.dimensions.1),
                ch,
                Color::White,
                Color::DarkBlue,
            )?;
        }

        Ok(())
    }

    fn draw_to_buffer_transition(&mut self, game_state: &GameState) -> Result<()> {
        let message = game_state.transition_message();
        let lines: Vec<&str> = message.split('\n').collect();
        let y_start = (self.dimensions.1 / 2) - (lines.len() as u16 / 2);

        for (i, line) in lines.iter().enumerate() {
            let x_start = (self.dimensions.0 - line.len() as u16) / 2;
            for (j, ch) in line.chars().enumerate() {
                self.set_buffer_cell(
                    &Point::new(x_start + j as u16, y_start + i as u16),
                    ch,
                    Color::White,
                    Color::DarkBlue,
                )?;
            }
        }

        Ok(())
    }

    fn draw_to_buffer_game_over(&mut self, game_state: &GameState, reason: GameEndReason) -> Result<()> {
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

        let bg_color = match reason {
            GameEndReason::Victory => Color::Green,
            GameEndReason::Collision => Color::Red,
        };

        let lines: Vec<&str> = message.split('\n').collect();
        let y_start = (self.dimensions.1 / 2) - (lines.len() as u16 / 2);

        for (i, line) in lines.iter().enumerate() {
            let x_start = (self.dimensions.0 - line.len() as u16) / 2;
            for (j, ch) in line.chars().enumerate() {
                self.set_buffer_cell(
                    &Point::new(x_start + j as u16, y_start + i as u16),
                    ch,
                    Color::White,
                    bg_color,
                )?;
            }
        }

        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}