use std::io::{Stdout, Write};

use anyhow::{Result, Context};
use crossterm::{QueueableCommand, style::{Color, SetForegroundColor, Print}, ExecutableCommand, cursor::MoveTo};

use crate::table::Table;

pub struct Terminal {
    enable_color: bool,
    stdout: Stdout,
    offset_x: u16,
    offset_y: u16,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        let mut stdout = std::io::stdout();
        stdout.execute(crossterm::terminal::EnterAlternateScreen)?;
        Ok(Self {
            enable_color: true,
            stdout,
            offset_x: 0,
            offset_y: 0,
        })
    }
}

impl Terminal {
    fn queue_set_foreground_color(&mut self, color: Color) -> Result<&mut Stdout>{
        if !self.enable_color {
            return Ok(&mut self.stdout);
        }
        self.stdout.queue(SetForegroundColor(color)).context(format!("could not set the foreground color to {:?}", color))
    }

    pub fn clear(&mut self) -> Result<&mut Stdout> {
        self.stdout.execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).context("could not clear the terminal")
    }

    pub fn queue_move_cursor(&mut self, x: u16, y: u16) -> Result<&mut Stdout> {
        let target_x = x + self.offset_x;
        let target_y = y + self.offset_y;
        self.stdout.queue(MoveTo(target_x, target_y)).context(format!("could not move the cursor to {}, {}", target_x, target_y))
    }

    pub fn queue_write_at_current_position(&mut self, text: &str, color: Color) -> Result<&mut Stdout> {
        self.queue_set_foreground_color(color)?.queue(Print(text)).context("could not write text at current position")
    }

    pub fn queue_write(&mut self, text: &str, color: Color, x: u16, y: u16) -> Result<&mut Stdout>{
        self.queue_write_raw(text, color, x + self.offset_x, y + self.offset_y)
    }

    pub fn queue_write_raw(&mut self, text: &str, color: Color, x: u16, y: u16) -> Result<&mut Stdout> {
        self.queue_set_foreground_color(color)?.queue(MoveTo(x, y))?.queue(Print(text)).context(format!("could not write at position {} {}", x, y))
    }

    pub fn write_table(&mut self, table: &Table) -> Result<()> {
        // TODO: Figure out how to draw the table in different corners
        let mut x_offset = 0;
        for col in table.columns().iter() {
            for (j, cell) in col.cells().iter().enumerate() {
                self.queue_write(format!("{:>width$}", &cell.text, width=(col.width + 1) as usize).as_str(), cell.color, x_offset, j as u16)?.flush()?;
            }
            x_offset += col.width + 1;
        }
        self.stdout.flush().context("could not flush stdout while writing table column")
    }
}
