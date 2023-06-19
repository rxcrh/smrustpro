use egui::{CentralPanel, Color32, Painter, Pos2, Stroke};
use rand::prelude::*;

pub(crate) struct Application {
    pos: Pos2,
    rng: rand::rngs::ThreadRng,
}

impl Application {
    fn generate_rand_pos(&mut self, maxwidth: f32, maxheight: f32) {
        self.pos = Pos2 {
            x: self.rng.gen_range(0. .. maxwidth),
            y: self.rng.gen_range(0. .. maxheight),
        };
    }
}

impl Default for Application {
    fn default() -> Self {
        Self {
            pos: Pos2::default(),
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
        if self.rng.gen::<f32>() < 0.01 { self.generate_rand_pos(resp.rect.width(), resp.rect.height()); }

        if let Some(pos) = resp.hover_pos() {
            painter.line_segment(
                [self.pos, pos],
                Stroke {
                    width: 2.0,
                    color: Color32::WHITE,
                },
            );
        }

        ctx.request_repaint();
    }
}
