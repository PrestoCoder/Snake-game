use crossterm::{
    style::{Color, SetForegroundColor, SetBackgroundColor},
    cursor::MoveTo,
    QueueableCommand,
};
use std::io::Stdout;
use crate::utils::Result;

pub struct DisplayManager {
    width: u16,
    height: u16,
}

impl DisplayManager {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    pub fn draw_centered_box(
        &self,
        stdout: &mut Stdout,
        text: &str,
        bg_color: Color,
    ) -> Result<()> {
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
                .queue(SetForegroundColor(Color::White))?
                .queue(crossterm::style::Print(line))?;
        }

        stdout.queue(SetBackgroundColor(Color::Reset))?;
        Ok(())
    }

    pub fn draw_status_bar(
        &self,
        stdout: &mut Stdout,
        text: &str,
        bg_color: Color,
    ) -> Result<()> {
        let x = 2;
        let y = self.height;

        stdout
            .queue(MoveTo(x, y))?
            .queue(SetForegroundColor(Color::White))?
            .queue(SetBackgroundColor(bg_color))?
            .queue(crossterm::style::Print(text))?
            .queue(SetBackgroundColor(Color::Reset))?;

        Ok(())
    }
}