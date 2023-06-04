use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode, MouseButton,
        MouseEvent, MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    style::*,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

enum Event<Key, Pos> {
    KeyInput(Key),
    LeftClick(Pos),
    Tick,
}

enum Mode {
    Insert,
    Play,
}

struct World {
    width: u16,
    height: u16,
    alive: Vec<(u16, u16)>,
}

pub mod defaults;

impl World {
    fn width(self, w: u16) -> Self {
        Self {
            width: w,
            height: self.height,
            alive: self.alive,
        }
    }
    fn height(self, h: u16) -> Self {
        Self {
            width: self.width,
            height: h,
            alive: self.alive,
        }
    }

    fn get_grid(&self, mode: &Mode, height: u16, width: u16) -> Vec<Spans> {
        let mut spans = vec![];

        for row in 0..height {
            spans.push(vec![Spans::from({
                let mut cols = vec![];
                for col in 0..width {
                    cols.push({
                        if self.alive.iter().any(|&x| x == (row, col)) {
                            Span::styled(
                                "█",
                                Style::default().fg({
                                    match mode {
                                        Mode::Insert => Color::Green,
                                        Mode::Play => Color::Red,
                                    }
                                }),
                            )
                        } else {
                            match mode {
                                Mode::Insert => Span::raw("."),
                                Mode::Play => Span::styled(
                                    "█",
                                    Style::default().add_modifier(Modifier::REVERSED),
                                ),
                            }
                        }
                    });
                }
                cols
            })])
        }
        return spans.into_iter().flatten().collect();
    }

    fn next_day(&mut self) {
        let alive_as_matrix = self.get_alives_as_matrix_with_puffer();

        for row in 1..self.height+1 {
            for col in 1..self.width+1 {
                let row = row as usize;
                let col = col as usize;

                if alive_as_matrix[row][col] == 0 && self.get_num_neighbours(&alive_as_matrix, row, col) == 3 {
                    self.alive.push((row as u16, col as u16));
                } else if alive_as_matrix[row][col] == 1
                    && self.get_num_neighbours(&alive_as_matrix, row, col) != 2
                    && self.get_num_neighbours(&alive_as_matrix, row, col) != 3
                {
                    self.alive
                        .retain(|&x| x.0 != row as u16 || x.1 != col as u16);
                }
            }
        }
    }

    fn get_num_neighbours(&self, m: &Vec<Vec<u32>>, row: usize, col: usize) -> u16 {
        let mut num_neighbours = 0;

        if m[row - 1][col - 1] == 1 {
            num_neighbours += 1
        }
        if m[row - 1][col] == 1 {
            num_neighbours += 1
        }
        if m[row - 1][col + 1] == 1 {
            num_neighbours += 1
        }
        if m[row][col - 1] == 1 {
            num_neighbours += 1
        }
        if m[row][col + 1] == 1 {
            num_neighbours += 1
        }
        if m[row + 1][col - 1] == 1 {
            num_neighbours += 1
        }
        if m[row + 1][col] == 1 {
            num_neighbours += 1
        }
        if m[row + 1][col + 1] == 1 {
            num_neighbours += 1
        }
        num_neighbours
    }

    fn get_alives_as_matrix_with_puffer(&self) -> Vec<Vec<u32>> {
        let mut alive_as_matrix = vec![vec![0; self.width as usize + 2]; self.height as usize + 2];
        for alive in self.alive.iter() {
            alive_as_matrix[alive.0 as usize][alive.1 as usize] = 1;
        }
        alive_as_matrix
    }

    fn remove_not_in_world(&mut self) {
        self.alive
            .retain(|&x| x.0 < self.height && x.1 < self.width);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(500);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                match event::read().expect("can read events") {
                    CEvent::Key(key) => tx.send(Event::KeyInput(key)).expect("can send keyevents"),
                    CEvent::Mouse(MouseEvent {
                        kind, column, row, ..
                    }) => match kind {
                        MouseEventKind::Down(MouseButton::Left) => tx
                            .send(Event::LeftClick((row, column)))
                            .expect("can send mouseevents"),
                        _ => {}
                    },
                    _ => {}
                };
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let mut stdout = io::stdout();

    execute!(stdout, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut should_play = false;
    let mut mode = Mode::Insert;
    let mut size = terminal.size()?;
    let mut world = World::default().width(size.width).height(size.height);

    loop {
        if should_play == true {
            world.next_day();
            world.remove_not_in_world();
        }

        terminal.draw(|f| {
            size = f.size();

            let world_grided = world.get_grid(&mode, size.height - 2, size.width - 2);

            let world_block = Paragraph::new(world_grided)
                .block(
                    Block::default()
                        .title({
                            match mode {
                                Mode::Insert => "Editor - Game of Life",
                                Mode::Play => "Conways - Game of Life",
                            }
                        })
                        .borders(Borders::ALL),
                )
                .wrap(Wrap { trim: true });

            if world.alive.is_empty() {
                mode = Mode::Insert;
                should_play = false;
            }
            f.render_widget(world_block, size);
        })?;

        match rx.recv()? {
            Event::KeyInput(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    terminal.clear()?;
                    break;
                }
                KeyCode::Delete => {
                    world.alive.clear();
                }
                KeyCode::Char(' ') => {
                    should_play = !should_play;
                }
                KeyCode::Char('i') => {
                    should_play = false;
                    mode = Mode::Insert;
                }
                KeyCode::Enter => {
                    should_play = true;
                    mode = Mode::Play;
                }
                KeyCode::Char('1') => {
                    should_play = true;
                    mode = Mode::Play;
                    world.alive = World::pulsar().alive;
                }
                _ => {}
            },
            Event::LeftClick(pos) => {
                if pos.0 as i32 - 1 < 0
                    || pos.0 as i32 - size.bottom() as i32 > 0
                    || pos.1 as i32 + 1 < 0
                    || pos.1 as i32 - size.right() as i32 > 0
                {
                    continue;
                }
                let position = (pos.0 - 1, pos.1 - 1);
                if !world
                    .alive
                    .iter()
                    .any(|x| x.0 == position.0 && x.1 == position.1)
                {
                    world.alive.push(position);
                } else {
                    world
                        .alive
                        .retain(|x| x.0 != position.0 || x.1 != position.1);
                }
            }
            Event::Tick => {}
        }
    }

    execute!(terminal.backend_mut(), DisableMouseCapture)?;
    terminal.show_cursor()?;
    terminal.clear()?;
    Ok(())
}
