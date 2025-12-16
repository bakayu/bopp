use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Stdout, Write, stdout};

pub struct Terminal {
    stdout: Stdout,
}

impl Terminal {
    pub fn new() -> io::Result<Self> {
        Ok(Self { stdout: stdout() })
    }

    pub fn enter(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(self.stdout, EnterAlternateScreen, cursor::Hide)?;
        Ok(())
    }

    pub fn exit(&mut self) -> io::Result<()> {
        execute!(self.stdout, LeaveAlternateScreen, cursor::Show)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn clear(&mut self) -> io::Result<()> {
        execute!(self.stdout, terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }

    pub fn size(&self) -> io::Result<(u16, u16)> {
        terminal::size()
    }

    pub fn should_quit(&self) -> io::Result<bool> {
        if event::poll(std::time::Duration::from_millis(0))? {
            if let Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) = event::read()?
            {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn stdout(&mut self) -> &mut Stdout {
        &mut self.stdout
    }
}
