// display.rs
use crossterm::{
    style::{Color, SetForegroundColor, SetBackgroundColor},
    cursor::MoveTo,
    QueueableCommand,
};
use std::io::{Stdout, Write}; 
use crate::utils::Result;
use crate::entities::Point;

pub struct DisplayManager {
    width: u16,
    height: u16,
}

impl DisplayManager {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    // New method to clear the entire screen
    pub fn clear_screen(&self, stdout: &mut Stdout) -> Result<()> {
        for y in 0..self.height {
            for x in 0..self.width {
                stdout
                    .queue(MoveTo(x, y))?
                    .queue(SetBackgroundColor(Color::Reset))?
                    .queue(SetForegroundColor(Color::Reset))?
                    .queue(crossterm::style::Print(" "))?;
            }
        }
        stdout.flush()?;
        Ok(())
    }

    // New method to draw a single character
    pub fn draw_char(
        &self,
        stdout: &mut Stdout,
        point: &Point,
        ch: char,
        fg: Color,
        bg: Color
    ) -> Result<()> {
        if point.x < self.width && point.y < self.height {
            stdout
                .queue(MoveTo(point.x, point.y))?
                .queue(SetForegroundColor(fg))?
                .queue(SetBackgroundColor(bg))?
                .queue(crossterm::style::Print(ch))?;
        }
        Ok(())
    }

    pub fn draw_centered_box(
        &self,
        stdout: &mut Stdout,
        text: &str,
        bg_color: Color,
        fg_color: Color,
    ) -> Result<()> {
        // Clear screen first
        self.clear_screen(stdout)?;

        let lines: Vec<&str> = text.split('\n').collect();
        let y_start = (self.height / 2) - (lines.len() as u16 / 2);

        let max_width = lines.iter().map(|line| line.len()).max().unwrap_or(0) as u16;
        let padding = 2;
        let box_width = max_width + (padding * 2);
        let box_height = lines.len() as u16 + (padding * 2);
        let box_x = (self.width - box_width) / 2;
        let box_y = y_start - padding;

        // Draw box background
        for y in 0..box_height {
            for x in 0..box_width {
                stdout
                    .queue(MoveTo(box_x + x, box_y + y))?
                    .queue(SetBackgroundColor(bg_color))?
                    .queue(crossterm::style::Print(" "))?;
            }
        }

        // Draw text
        for (i, line) in lines.iter().enumerate() {
            let x = (self.width - line.len() as u16) / 2;
            let y = y_start + i as u16;

            stdout
                .queue(MoveTo(x, y))?
                .queue(SetForegroundColor(fg_color))?
                .queue(SetBackgroundColor(bg_color))?
                .queue(crossterm::style::Print(line))?;
        }

        stdout.queue(SetBackgroundColor(Color::Reset))?;
        stdout.queue(SetForegroundColor(Color::Reset))?;
        stdout.flush()?;
        Ok(())
    }

    pub fn draw_status_bar(
        &self,
        stdout: &mut Stdout,
        text: &str,
        bg_color: Color,
        fg_color: Color,
    ) -> Result<()> {
        // Fill entire bottom line with background color
        for x in 0..self.width {
            stdout
                .queue(MoveTo(x, self.height))?
                .queue(SetBackgroundColor(bg_color))?
                .queue(crossterm::style::Print(" "))?;
        }

        // Draw the text
        let x = 2;
        stdout
            .queue(MoveTo(x, self.height))?
            .queue(SetForegroundColor(fg_color))?
            .queue(SetBackgroundColor(bg_color))?
            .queue(crossterm::style::Print(text))?;

        stdout.queue(SetBackgroundColor(Color::Reset))?;
        stdout.queue(SetForegroundColor(Color::Reset))?;
        stdout.flush()?;
        Ok(())
    }
}