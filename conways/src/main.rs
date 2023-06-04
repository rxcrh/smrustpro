use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    layout::*,
    style::*,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

enum Event<I> {
    Input(I),
    Tick,
}

struct World {
    width: u16,
    height: u16,
    alive: Vec<(u16, u16)>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            // Mid Point: (10, 20)
            // [TODO] find a better way to add new automata
            width: 41,
            height: 21,
            alive: vec![
                (9, 18),
                (9, 17),
                (9, 16),
                (9, 22),
                (9, 23),
                (9, 24),
                (11, 18),
                (11, 17),
                (11, 16),
                (11, 22),
                (11, 23),
                (11, 24),
                (8, 19),
                (7, 19),
                (6, 19),
                (12, 19),
                (13, 19),
                (14, 19),
                (8, 21),
                (7, 21),
                (6, 21),
                (12, 21),
                (13, 21),
                (14, 21),
                (4, 16),
                (4, 17),
                (4, 18),
                (4, 22),
                (4, 23),
                (4, 24),
                (16, 16),
                (16, 17),
                (16, 18),
                (16, 22),
                (16, 23),
                (16, 24),
                (8, 14),
                (7, 14),
                (6, 14),
                (12, 14),
                (13, 14),
                (14, 14),
                (8, 26),
                (7, 26),
                (6, 26),
                (12, 26),
                (13, 26),
                (14, 26),
            ],
        }
    }
}

impl World {
    fn get_grid(&self, height: u16, width: u16) -> Vec<Spans> {
        let mut spans = vec![];

        let row_height = height / self.height as u16;
        let col_width = width / self.width as u16;

        for row in 0..self.height {
            spans.push(vec![
                Spans::from(
                    vec![{
                        let mut cols = vec![];
                        for col in 0..self.width {
                            cols.push(vec![
                                {
                                    if self.alive.iter().any(|&x| x == (row, col)) {
                                        Span::raw("█")
                                    } else {
                                        Span::styled(
                                            "█",
                                            Style::default().add_modifier(Modifier::REVERSED),
                                        )
                                    }
                                };
                                col_width as usize
                            ]);
                        }
                        cols.into_iter().flatten().collect::<Vec<Span>>()
                    }]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<Span>>()
                );
                row_height as usize
            ])
        }
        return spans.into_iter().flatten().collect();
    }

    fn next_day(&mut self) {
        let mut alive_as_matrix = vec![vec![0; self.width as usize]; self.height as usize];
        for alive in self.alive.iter() {
            alive_as_matrix[alive.0 as usize][alive.1 as usize] = 1;
        }

        for row in 0..self.height {
            for col in 0..self.width {
                let row = row as usize;
                let col = col as usize;

                if alive_as_matrix[row][col] == 0
                    && get_num_neighbours(&alive_as_matrix, row, col) == 3
                {
                    self.alive.push((row as u16, col as u16));
                } else if alive_as_matrix[row][col] == 1
                    && get_num_neighbours(&alive_as_matrix, row, col) != 2
                    && get_num_neighbours(&alive_as_matrix, row, col) != 3
                {
                    self.alive
                        .retain(|&x| x.0 != row as u16 || x.1 != col as u16);
                }
            }
        }
    }

    fn remove_not_in_world(&mut self) {
        self.alive
            .retain(|&x| x.0 < self.height && x.1 < self.width);
    }
}

fn get_num_neighbours(m: &Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    let mut num_neighbours = 0;
    if is_in_bounds(i as i32 - 1, j as i32 - 1, m.len(), m[0].len()) && m[i - 1][j - 1] == 1 {
        num_neighbours += 1
    }
    if is_in_bounds(i as i32 - 1, j as i32, m.len(), m[0].len()) && m[i - 1][j] == 1 {
        num_neighbours += 1
    }
    if is_in_bounds(i as i32 - 1, j as i32 + 1, m.len(), m[0].len()) && m[i - 1][j + 1] == 1 {
        num_neighbours += 1
    }
    if is_in_bounds(i as i32, j as i32 - 1, m.len(), m[0].len()) && m[i][j - 1] == 1 {
        num_neighbours += 1
    }
    if is_in_bounds(i as i32, j as i32 + 1, m.len(), m[0].len()) && m[i][j + 1] == 1 {
        num_neighbours += 1
    }
    if is_in_bounds(i as i32 + 1, j as i32 - 1, m.len(), m[0].len()) && m[i + 1][j - 1] == 1 {
        num_neighbours += 1
    }
    if is_in_bounds(i as i32 + 1, j as i32, m.len(), m[0].len()) && m[i + 1][j] == 1 {
        num_neighbours += 1
    }
    if is_in_bounds(i as i32 + 1, j as i32 + 1, m.len(), m[0].len()) && m[i + 1][j + 1] == 1 {
        num_neighbours += 1
    }
    num_neighbours
}

fn is_in_bounds(i: i32, j: i32, i_len: usize, j_len: usize) -> bool {
    i >= 0 && i < i_len as i32 && j >= 0 && j < j_len as i32
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
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut should_play = true;
    let mut world = World::default();

    loop {
        if should_play == true {
            terminal.draw(|f| {
                let size = f.size();

                let viewport_height = size.height - 2;
                let viewport_width = size.width - 2;

                if size.width < viewport_width || size.height < viewport_height {
                    todo!();
                }

                let vertical_remainder = viewport_height % world.height;
                let horizontal_remainder = viewport_width % world.width;

                let world_grided = world.get_grid(
                    viewport_height - vertical_remainder,
                    viewport_width - horizontal_remainder,
                );

                let chunks = Layout::default()
                    .vertical_margin(vertical_remainder / 2)
                    .horizontal_margin(horizontal_remainder / 2)
                    .constraints([Constraint::Length(size.width - 2), Constraint::Min(2)].as_ref())
                    .split(size);

                let block = Paragraph::new(world_grided)
                    .block(
                        Block::default()
                            .title("Conways - Game of Life")
                            .borders(Borders::ALL),
                    )
                    .wrap(Wrap { trim: true });

                let dead_text = vec![
                    vec![
                        Spans::default();
                        (viewport_height - vertical_remainder) as usize / 2 as usize
                    ],
                    vec![Spans::from(vec![Span::raw("Everything is dead!")])],
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<Spans>>();

                let end_screen = Paragraph::new(dead_text)
                    .block(
                        Block::default()
                            .title("Conways - Game of Life")
                            .borders(Borders::ALL),
                    )
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: true });

                if !world.alive.is_empty() {
                    f.render_widget(block, chunks[0]);
                } else {
                    f.render_widget(end_screen, chunks[0]);
                }
            })?;
        }

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    terminal.clear()?;
                    break;
                }
                KeyCode::Char(' ') => {
                    if should_play == true {
                        should_play = false;
                    } else {
                        should_play = true;
                    }
                }
                _ => {}
            },
            Event::Tick => {}
        }

        world.next_day();
        world.remove_not_in_world();
    }
    Ok(())
}
