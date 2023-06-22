use egui::{CentralPanel, Color32, Painter, Pos2, Stroke};
use rand::prelude::*;

pub(crate) struct Application {
    first: bool,
    pos: Pos2,
    directions: Vec<Pos2>,
    rng: rand::rngs::ThreadRng,
}

impl Application {
    fn generate_rand_pos(&mut self, maxwidth: f32, maxheight: f32) -> Pos2 {
        Pos2 {
            x: self.rng.gen_range(0. ..maxwidth),
            y: self.rng.gen_range(0. ..maxheight),
        }
    }

    fn move_towards_nearest(&mut self) {
        let distances = self
            .directions
            .iter()
            .map(|&x| self.pos.distance(x))
            .collect::<Vec<f32>>();
        let nearest_point_ind = distances
            .iter()
            .zip(0..self.directions.len())
            .min_by_key(|y| y.1)
            .unwrap()
            .1;
        let nearest_point = self.directions[nearest_point_ind];

        self.pos = nearest_point.lerp(self.pos, 0.99);
    }
}

impl Default for Application {
    fn default() -> Self {
        Self {
            first: true,
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
            self.directions = vec![self.generate_rand_pos(resp.rect.width(), resp.rect.height()); 10];
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

        self.move_towards_nearest();
        ctx.request_repaint();
    }
}
