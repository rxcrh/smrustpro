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
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

struct World {
    width: u64,
    height: u64,
    alive: Vec<(u64, u64)>,
}

impl World {
    fn get_grid(&self, height: u16, width: u16) -> Vec<Spans> {
        let mut spans = vec![];

        // accounting for borders
        let height = height - 2;
        let width = width - 2;

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
                                        Span::raw("#")
                                    } else {
                                        Span::raw(".")
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
        if self.alive.is_empty() {
            self.alive.push((4,4));
        } else {
            self.alive.clear();
        }
    }
}

enum Event<I> {
    Input(I),
    Tick,
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
    #[allow(unused_mut)]
    let mut world = World {
        width: 30,
        height: 20,
        alive: vec![(0,0), (10, 10), (4, 5), (19, 29)],
    };

    loop {
        if should_play == true {
            //world.next_day();
            terminal.draw(|f| {
                let size = f.size();
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Length(size.width), Constraint::Min(0)].as_ref())
                    .split(size);

                let world = world.get_grid(size.height, size.width);

                let block = Paragraph::new(world)
                    .block(
                        Block::default()
                            .title("Conways - Game of Life")
                            .borders(Borders::ALL),
                    )
                    .wrap(Wrap { trim: true });

                f.render_widget(block, chunks[0]);
            })?;
        }

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
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
    }
    Ok(())
}
