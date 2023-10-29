use egui::{CentralPanel, Color32, Painter, Pos2, Stroke};
use rand::prelude::*;

pub(crate) enum Mode {
    Mouse,
    Furthest,
}

pub(crate) struct Application {
    pub mode: Mode,
    pub first: bool,
    pub pos: Pos2,
    pub directions: Vec<Pos2>,
    pub rng: rand::rngs::ThreadRng,
}


impl Application {
    fn generate_rand_pos(&mut self, maxwidth: f32, maxheight: f32) -> Pos2 {
        Pos2 {
            x: self.rng.gen_range(0. ..maxwidth),
            y: self.rng.gen_range(0. ..maxheight),
        }
    }

    fn move_towards_furthest(&mut self) {
        let distances = self
            .directions
            .iter()
            .map(|&x| self.pos.distance(x))
            .collect::<Vec<f32>>();
        let furthest_id = distances
            .iter()
            .zip(0..self.directions.len())
            .max_by_key(|y| y.1)
            .unwrap()
            .1;
        let furthest_point = self.directions[furthest_id];

        self.pos = furthest_point.lerp(self.pos, 0.99);
    }
}

impl Default for Application {
    fn default() -> Self {
        Self {
            first: true,
            mode: Mode::Mouse,
            pos: Pos2 { x: 0., y: 0. },
            directions: vec![],
            rng: rand::thread_rng(),
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let resp = CentralPanel::default()
            .show(ctx, |ui| {
                ui.heading("Distancer");
            })
            .response;
        let painter = Painter::new(ctx.clone(), resp.layer_id, resp.rect);

        if self.first {
            for _ in 0..10 {
                let new_entry = self.generate_rand_pos(resp.rect.width(), resp.rect.height());
                self.directions.push(new_entry);
            }
            self.first = false;
        }

        if self.rng.gen::<f32>() < 0.01 {
            let new_pos = self.generate_rand_pos(resp.rect.width(), resp.rect.height());
            self.directions.push(new_pos);
            self.directions.remove(0);
        }

        self.directions.iter().for_each(|&listpos| {
            painter.line_segment(
                [self.pos, listpos.lerp(self.pos, 0.9)],
                Stroke {
                    width: 2.0,
                    color: Color32::WHITE,
                },
            )
        });
        
        match self.mode {
            Mode::Furthest => self.move_towards_furthest(),
            Mode::Mouse => {
                if let Some(cursor_pos) = ctx.pointer_latest_pos() {
                    self.pos = cursor_pos;
                }            
            }
        }
        ctx.request_repaint();
    }
}
