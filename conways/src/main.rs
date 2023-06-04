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
    layout::*,
    style::*,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

enum Event<Key, Pos> {
    KeyInput(Key),
    MouseInput(Pos),
    Tick,
}

enum Mode {
    Insert,
    Play,
}

struct World {
    width: u16,
    height: u16,
    input: Vec<(u16, u16)>,
    alive: Vec<(u16, u16)>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            width: 41,
            height: 21,
            input: vec![(9, 9)],
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
    fn get_input_grid(&self) -> Vec<Spans> {
        let mut spans = vec![];

        for row in 0..self.height - 2 {
            spans.push(vec![Spans::from({
                let mut cols = vec![];
                for col in 0..self.width - 2 {
                    cols.push({
                        if self.input.iter().any(|&x| x == (row, col)) {
                            Span::styled("█", Style::default().fg(Color::Green))
                        } else {
                            Span::styled(".", Style::default())
                        }
                    });
                }
                cols
            })])
        }
        return spans.into_iter().flatten().collect();
    }

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
                                        Span::styled("█", Style::default().fg(Color::Green))
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
    let tick_rate = Duration::from_millis(2000);
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
                            .send(Event::MouseInput((row, column)))
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

    let mut should_play = true;
    let mut mode = Mode::Insert;
    let mut view = Rect::default();
    let mut world = World::default();

    loop {
        world.next_day();
        world.remove_not_in_world();
        match mode {
            Mode::Play => {
                if should_play == true {
                    terminal.draw(|f| {
                        let size = f.size();

                        let viewport_height = size.height - 2;
                        let viewport_width = size.width - 2;

                        let vertical_remainder = viewport_height % world.height;
                        let horizontal_remainder = viewport_width % world.width;

                        let world_grided = world.get_grid(
                            viewport_height - vertical_remainder,
                            viewport_width - horizontal_remainder,
                        );

                        let chunks = Layout::default()
                            .vertical_margin(vertical_remainder / 2)
                            .horizontal_margin(horizontal_remainder / 2)
                            .constraints(
                                [Constraint::Length(size.width - 2), Constraint::Min(2)].as_ref(),
                            )
                            .split(size);

                        let world_block = Paragraph::new(world_grided)
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
                            view = chunks[0];
                            f.render_widget(world_block, chunks[0]);
                        } else {
                            view = chunks[0];
                            f.render_widget(end_screen, chunks[0]);
                        }
                    })?;
                }
            }

            Mode::Insert => {
                terminal.draw(|f| {
                    let size = f.size();

                    let grided_input = world.get_input_grid();

                    let chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .vertical_margin(size.height / 2 - world.height / 2)
                        .horizontal_margin(size.width / 2 - world.width / 2)
                        .constraints([Constraint::Length(world.width)].as_ref())
                        .split(size);

                    let input_block = Paragraph::new(grided_input)
                        .block(
                            Block::default()
                                .title("Editor - Game of Life")
                                .borders(Borders::ALL),
                        )
                        .wrap(Wrap { trim: true });
                    view = chunks[0];
                    f.render_widget(input_block, chunks[0]);
                })?;
            }
        }

        match rx.recv()? {
            Event::KeyInput(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    terminal.clear()?;
                    break;
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

                    world.alive = world.input.clone();
                }
                KeyCode::Char('d') => {
                    should_play = true;
                    mode = Mode::Play;
                    world = World::default();
                }
                _ => {}
            },
            Event::MouseInput(pos) => {
                if pos.0 as i32 - (view.top() as i32 + 1) < 0
                    || pos.0 as i32 + (view.width as i32 - 1) < 0
                    || pos.1 as i32 - (view.left() as i32 + 1) < 0
                    || pos.1 as i32 - (view.height as i32 - 1) < 0
                {
                    continue;
                }
                world
                    .input
                    .push((pos.0 - (view.top() + 1), pos.1 - (view.left() + 1)));
            }
            Event::Tick => {}
        }
    }

    execute!(terminal.backend_mut(), DisableMouseCapture)?;
    terminal.show_cursor()?;
    terminal.clear()?;
    Ok(())
}
