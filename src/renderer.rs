use crate::terminal::Terminal;
use crossterm::{cursor, execute};
use std::io::{self, Write};

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&mut self, terminal: &mut Terminal, frame: &str, info: &str) -> io::Result<()> {
        terminal.clear()?;

        let (width, height) = terminal.size()?;
        let lines: Vec<&str> = frame.lines().collect();

        let frame_height = lines.len() as u16;
        let frame_width = lines.iter().map(|l| l.len()).max().unwrap_or(0) as u16;

        let start_y = height.saturating_sub(frame_height + 3) / 2;
        let start_x = width.saturating_sub(frame_width) / 2;

        for (i, line) in lines.iter().enumerate() {
            execute!(
                terminal.stdout(),
                cursor::MoveTo(start_x, start_y + i as u16)
            )?;
            write!(terminal.stdout(), "{}", line)?;
        }

        let info_y = start_y + frame_height + 2;
        let info_x = width.saturating_sub(info.len() as u16) / 2;
        execute!(terminal.stdout(), cursor::MoveTo(info_x, info_y))?;
        write!(terminal.stdout(), "{}", info)?;

        terminal.stdout().flush()?;
        Ok(())
    }
}
