use crossterm::{
    cursor::{self, MoveTo, MoveLeft},
    event::{KeyCode, KeyEvent, KeyEventKind},
    execute, queue,
    style::Print,
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::{io, time::Duration};

use crate::mode::Mode;
use crate::commands;

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
                        KeyCode::Enter => Self::draw_new_line(),
                        KeyCode::Backspace => Self::delete_back(),
                        KeyCode::Char(char) => {
                            let _ = run!(Print(char.to_string()));
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
                                let _ = run!(MoveTo(0, terminal::size()?.1 - 1), Clear(ClearType::CurrentLine), Print("!"));
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
            Mode::Command => {
                disable_raw_mode().unwrap();
                let mut command_buffer = String::new();
                io::stdin().read_line(&mut command_buffer).unwrap();

                //execute!(io::stdout(), Print(command_buffer.clone())).unwrap();
    
                commands::execute_command(command_buffer.into());

                let (x, y) = self.cursor_pos_editor;
                let _ = run!(MoveTo(x, y));
                self.mode = Mode::Normal;
            }
        }
    }

    pub(crate) fn draw_active_mode(&self) {
        let (x, y) = cursor::position().unwrap();
        // TODO only redraw if mode changed
        let _ = run!(
            MoveTo(0, terminal::size()?.1 - 2),
            Clear(ClearType::CurrentLine),
            Print(self.mode.stringify()),
            MoveTo(x, y)
        );
    }

    fn draw_new_line() {
        let (_, row) = cursor::position().unwrap();
        let _ = run!(MoveTo(0, row + 1));
    }

    fn delete_back() {
        let _ = run!(MoveLeft(1), Print(" "), MoveLeft(1));
    }
}
