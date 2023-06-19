use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let resp = egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.heading("JO");
            })
            .response;
        
        let painter = egui::Painter::new(ctx.clone(), resp.layer_id, resp.rect);

        if let Some(pos) = resp.hover_pos() {
            painter.line_segment([egui::Pos2::default(), pos], egui::Stroke {width: 2.0, color: egui::Color32::WHITE});
        }
    }
}
