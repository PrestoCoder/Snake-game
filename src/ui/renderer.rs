// renderer.rs
use crate::{
    utils::Result,
    core::GameState,
    entities::{Point, Obstacle},
    gameplay::{GameState as GameStateEnum, GameEndReason},
    ui::DisplayManager,
    config::BORDER_THICKNESS,
};
use crossterm::{
    cursor::{Hide, Show},
    execute,
    style::Color,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
};
use std::io::{stdout, Stdout};

pub struct Renderer {
    dimensions: (u16, u16),
    stdout: Stdout,
    display_manager: DisplayManager,
}

impl Renderer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            dimensions: (width, height),
            stdout: stdout(),
            display_manager: DisplayManager::new(width, height),
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
        match game_state.game_state() {
            GameStateEnum::Playing => {
                self.display_manager.clear_screen(&mut self.stdout)?;
                
                // Draw borders
                self.draw_borders()?;
                
                // Draw obstacles
                self.draw_obstacles(game_state.obstacles())?;
                
                // Draw snake
                for point in game_state.snake().body() {
                    self.display_manager.draw_char(&mut self.stdout, point, '█', Color::Green, Color::Reset)?;
                }
                
                // Draw food
                self.display_manager.draw_char(
                    &mut self.stdout, 
                    game_state.food().position(), 
                    '●', 
                    Color::Red, 
                    Color::Reset
                )?;
                
                // Draw status
                self.draw_status(game_state)?;
            }
            GameStateEnum::LevelTransition => {
                self.draw_transition(game_state)?;
            }
            GameStateEnum::GameOver(reason) => {
                self.draw_game_over(game_state, reason)?;
            }
        }
        Ok(())
    }

    fn draw_borders(&mut self) -> Result<()> {
        for y in 0..BORDER_THICKNESS {
            for x in 0..self.dimensions.0 {
                self.display_manager.draw_char(
                    &mut self.stdout,
                    &Point::new(x, y),
                    '█',
                    Color::Blue,
                    Color::Blue
                )?;
                self.display_manager.draw_char(
                    &mut self.stdout,
                    &Point::new(x, self.dimensions.1 - 1 - y),
                    '█',
                    Color::Blue,
                    Color::Blue
                )?;
            }
        }

        for x in 0..BORDER_THICKNESS {
            for y in 0..self.dimensions.1 {
                self.display_manager.draw_char(
                    &mut self.stdout,
                    &Point::new(x, y),
                    '█',
                    Color::Blue,
                    Color::Blue
                )?;
                self.display_manager.draw_char(
                    &mut self.stdout,
                    &Point::new(self.dimensions.0 - 1 - x, y),
                    '█',
                    Color::Blue,
                    Color::Blue
                )?;
            }
        }
        Ok(())
    }

    fn draw_obstacles(&mut self, obstacles: &[Obstacle]) -> Result<()> {
        for obstacle in obstacles {
            for point in &obstacle.blocks {
                self.display_manager.draw_char(
                    &mut self.stdout,
                    point,
                    '█',
                    Color::DarkGrey,
                    Color::DarkGrey
                )?;
            }
        }
        Ok(())
    }

    fn draw_status(&mut self, game_state: &GameState) -> Result<()> {
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

        self.display_manager.draw_status_bar(
            &mut self.stdout,
            &stats_text,
            Color::DarkBlue,
            Color::White,
        )
    }

    fn draw_transition(&mut self, game_state: &GameState) -> Result<()> {
        self.display_manager.draw_centered_box(
            &mut self.stdout,
            game_state.transition_message(),
            Color::DarkBlue,
            Color::White,
        )
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

        let bg_color = match reason {
            GameEndReason::Victory => Color::Green,
            GameEndReason::Collision => Color::Red,
        };

        self.display_manager.draw_centered_box(
            &mut self.stdout,
            &message,
            bg_color,
            Color::White,
        )
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}