use crossterm::{
    cursor::{self, MoveTo},
    event::{KeyCode, KeyEvent, KeyEventKind},
    execute, queue,
    style::Print,
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::{io, time::Duration};

use crate::mode::Mode;

const EDIT: char = 'e';
const QUIT: char = 'q';
const COMMAND: char = '!';

pub(crate) struct Session {
    mode: Mode,
    pub should_quit: bool,
    cursor_pos_editor: (u16, u16),
}

impl Session {
    pub(crate) fn new() -> Self {
        Self {
            mode: Mode::Normal,
            should_quit: false,
            cursor_pos_editor: (0, 0),
        }
    }

    pub(crate) fn handle_key_event(&mut self, key_event: KeyEvent) {
        let mut stdout = io::stdout();
        let KeyEvent {
            code,
            modifiers: _,
            kind,
            state: _,
        } = key_event;
        match self.mode {
            Mode::Edit => {
                if kind == KeyEventKind::Press {
                    match code {
                        KeyCode::Esc => self.mode = Mode::Normal,
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
                        KeyCode::Char(char) => match char {
                            QUIT => self.should_quit = true,
                            EDIT => self.mode = Mode::Edit,
                            COMMAND => {
                                self.mode = Mode::Command;
                                self.cursor_pos_editor = cursor::position().unwrap();
                                let _ = execute!(
                                    stdout,
                                    MoveTo(0, terminal::size()?.1 - 1),
                                    Print("!")
                                );
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
            Mode::Command => {
                let _ = execute!(stdout, Print("a"));
                
                std::thread::sleep(Duration::from_secs(1));
                let (x, y) = self.cursor_pos_editor;
                let _ = execute!(stdout, MoveTo(x, y));
                self.mode = Mode::Normal;
            }
        }
    }

    pub(crate) fn draw_active_mode(&self) {
        let mut stdout = io::stdout();
        let (x, y) = cursor::position().unwrap();
        let _ = run!(
            stdout,
            MoveTo(0, terminal::size()?.1 - 2),
            Clear(ClearType::CurrentLine),
            Print(self.mode.stringify()),
            MoveTo(x, y)
        );
    }
}
