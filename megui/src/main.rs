use eframe::egui;
use egui::*;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Spacetester",
        options,
        Box::new(|_cc| Box::<Content>::default()),
    )
}

#[derive(Default)]
struct Content {
    text: String,
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Press/Hold/Release. Press your Spacebar to test.");
                });
            });
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .max_height(ui.available_height() - 30.)
                .show(ui, |ui| {
                    ui.label(&self.text);
                });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                if ui.button("Clear").clicked() {
                    self.text.clear();
                }
            });
            if ctx.input(|i| i.key_pressed(Key::Space)) {
                self.text.push_str("\nPressed");
            }
            if ctx.input(|i| i.key_down(Key::Space)) {
                self.text.push_str("\nHeld");
                ui.ctx().request_repaint(); // make sure we note the holding.
            }
            if ctx.input(|i| i.key_released(Key::Space)) {
                self.text.push_str("\nReleased");
            }
        });
    }
}
