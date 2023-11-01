use crossterm::{
    cursor::{self, MoveTo},
    event::{self, KeyCode, KeyEvent, KeyEventKind},
    execute, queue,
    style::Print,
    terminal::{self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
};
use std::{
    io::{self, Result, Write},
    time::Duration,
};

macro_rules! run {
    ($buffer:ident $(, $command:ident($($tokens:tt)*))+) => {
        queue!($buffer $(, $command($($tokens)*))*).map_err(|err| {
            eprintln!("[ERROR] Something went wrong: {err}");
        })
    };
}

const QUIT: char = 'q';
const EDIT: char = 'e';
const COMMAND: char = '!';

enum Mode {
    Edit,
    Normal,
    Command,
}

impl Mode {
    fn stringify(&self) -> String {
        match self {
            Mode::Edit => "Edit".to_string(),
            Mode::Normal => "Command".to_string(),
            Mode::Command => "".to_string(),
        }
    }
}

fn handle_key_event(mode: &mut Mode, key_event: KeyEvent) -> bool {
    let mut should_quit = false;
    let mut stdout = io::stdout();
    let KeyEvent {
        code,
        modifiers: _,
        kind,
        state: _,
    } = key_event;
    match mode {
        Mode::Edit => {
            if kind == KeyEventKind::Press {
                match code {
                    KeyCode::Esc => *mode = Mode::Normal,
                    KeyCode::Enter => {
                        let (_, row) = cursor::position().unwrap();
                        let _ = run!(stdout, MoveTo(0, row + 1));
                    }
                    KeyCode::Char(char) => {
                        let _ = run!(stdout, Print(char.to_string()));
                    }
                    _ => {}
                }
            }
        }
        Mode::Normal => {
            enable_raw_mode().unwrap();
            if kind == KeyEventKind::Press {
                match code {
                    KeyCode::Char(char) => {
                        match char {
                            QUIT => {should_quit = true},
                            EDIT  => {*mode = Mode::Edit},
                            COMMAND => {
                                *mode = Mode::Command;
                                let (x, y) = cursor::position().unwrap();
                                let _ = execute!(stdout, MoveTo(0, terminal::size()?.1-1), Print("!"));

                                std::thread::sleep(Duration::from_secs(1));
                  
                                let _ = execute!(stdout, MoveTo(x,y));
                                *mode = Mode::Normal;
                            },
                            _ => {},
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
    should_quit
}

fn draw_mode(mode: &Mode) {
    let mut stdout = io::stdout();
    let (x, y) = cursor::position().unwrap();
    let _ = run!(stdout, MoveTo(0, terminal::size()?.1-2), Clear(ClearType::CurrentLine), Print(mode.stringify()), MoveTo(x,y));
}

fn main() -> Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, MoveTo(0, 0))?;
    enable_raw_mode()?;

    let mut mode = Mode::Normal;

    loop {
        if event::poll(Duration::from_millis(150))? {
            match event::read()? {
                event::Event::Key(key_event) => {
                    if handle_key_event(&mut mode, key_event) {break}
                }
                _ => {}
            }
        }
        draw_mode(&mode);
        stdout.flush()?;
    }

    execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
