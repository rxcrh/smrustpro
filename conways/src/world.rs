use fallible_iterator::FallibleIterator;
use rusqlite::{Connection, Result};
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
};
use crate::Mode;

pub struct World {
    pub width: u16,
    pub height: u16,
    pub alive: Vec<(u16, u16)>,
}

impl World {
    pub fn width(self, w: u16) -> Self {
        Self {
            width: w,
            height: self.height,
            alive: self.alive,
        }
    }
    pub fn height(self, h: u16) -> Self {
        Self {
            width: self.width,
            height: h,
            alive: self.alive,
        }
    }

    pub fn get_grid(&self, mode: &Mode, height: u16, width: u16) -> Vec<Spans> {
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
                                        _ => Color::Red,
                                    }
                                }),
                            )
                        } else {
                            match mode {
                                Mode::Insert => Span::raw("."),
                                _ => Span::styled(
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

    pub fn next_day(&mut self) {
        let alive_as_matrix = self.get_alives_as_matrix_with_puffer();

        for row in 1..self.height + 1 {
            for col in 1..self.width + 1 {
                let row = row as usize;
                let col = col as usize;

                if alive_as_matrix[row][col] == 0
                    && self.get_num_neighbours(&alive_as_matrix, row, col) == 3
                {
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

        self.remove_not_in_world();
    }

    pub fn load_alive(&self, conn: &Connection) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut stmt = conn.prepare("SELECT alive FROM templates")?;
        let rows = stmt.query_map([], |row| row.get(0))?;
    
        let mut entrys = Vec::new();
        for entry in rows {
            entrys.push(entry?);
        }
        Ok(entrys)
    }

    pub fn save_current_state(&self, conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
        conn.execute(
            "INSERT INTO templates (width, height, alive) VALUES (?1, ?2, ?3)",
            (&self.width, &self.height, &self.alive_to_string()),
        )?;
        Ok(())
    }

    fn alive_to_string(&self) -> String {
        self.alive
            .iter()
            .map(|(x, y)| format!("{},{}", x.to_string(), y.to_string()))
            .collect::<Vec<String>>()
            .join(":")
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
