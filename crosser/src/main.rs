use crossterm::{
    execute, queue,
    cursor,
    event::{self, KeyCode, KeyEventKind},
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, ScrollDown},
};
use std::{
    io::{self, Result, Write},
    time::Duration,
};

fn main() -> Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    loop {
        if event::poll(Duration::from_millis(150))? {
            match event::read()? {
                event::Event::Key(key_event) => {
                    if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q')
                    {
                        break;
                    }
                }
                _ => {
                }
            }
            queue!(stdout, cursor::MoveTo(10, 10), Print("fasdf".to_string()))?;
        }
        stdout.flush()?;
    }

    execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
