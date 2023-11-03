use crossterm::{
    cursor::MoveTo,
    event,
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::{
    io::{self, Result, Write},
    time::Duration,
};

macro_rules! run {
    ($($command:expr) ,*) => {
        queue!(io::stdout() $(, $command)*).map_err(|err| {
            eprintln!("[ERROR] Something went wrong: {err}");
        })
    };
}

mod mode;
mod session;
mod commands;

fn main() -> Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, MoveTo(0, 0))?;
    enable_raw_mode()?;

    let mut session = session::Session::new();

    loop {
        session.draw_active_mode();
        if event::poll(Duration::from_millis(150))? {
            match event::read()? {
                event::Event::Key(key_event) => {
                    session.handle_key_event(key_event);
                    if session.should_quit == true {
                        break;
                    }
                }
                _ => {}
            }
        }
        stdout.flush()?;
    }

    execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
