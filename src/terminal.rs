use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Stdout, stdout};

pub struct Terminal {
    stdout: Stdout,
}

pub enum TerminalEvent {
    Quit,
    SwitchAnimation(usize),
    None,
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

    pub fn poll_event(&self) -> io::Result<TerminalEvent> {
        if event::poll(std::time::Duration::from_millis(0))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => {
                    return Ok(TerminalEvent::Quit);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char(c @ '1'..='9'),
                    ..
                }) => {
                    let index = c.to_digit(10).unwrap() as usize - 1;
                    return Ok(TerminalEvent::SwitchAnimation(index));
                }
                _ => {}
            }
        }
        Ok(TerminalEvent::None)
    }

    pub fn stdout(&mut self) -> &mut Stdout {
        &mut self.stdout
    }
}
