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
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

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

    loop {
        if should_play == true {
            terminal.draw(|f| {
                let size = f.size();
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Length(size.width), Constraint::Min(0)].as_ref())
                    .split(size);

                let text = vec![
                    Spans::from(vec![
                        Span::raw("First"),
                        Span::styled("line", Style::default().add_modifier(Modifier::ITALIC)),
                        Span::raw("."),
                    ]),
                    Spans::from(Span::styled("Second line", Style::default().fg(Color::Red))),
                ];
                let block = Paragraph::new(text)
                    .block(Block::default().title("Conways - Game of Life").borders(Borders::ALL))
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: true });

                f.render_widget(block, chunks[0]);
            })?;
        } else {
            terminal.clear()?;
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
