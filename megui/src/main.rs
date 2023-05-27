use std::path::Path;

use image;
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
    num_tiers: u32,
    tiers: Vec<String>,
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let tier_size: f32 = 0.05 * ui.available_width();

            let texture = ui.ctx().load_texture(
                "texture",
                load_image_from_path(Path::new("src/assets/nand_placeholder.png")).unwrap(),
                Default::default(),
            );

            ui.add(egui::Image::new(&texture, texture.size_vec2()));

            ui.horizontal(|ui| {
                ui.heading("Rustier");
                if ui.button("+").clicked() {
                    self.num_tiers += 1;
                    self.tiers.push(self.num_tiers.to_string().to_owned());
                }
            });

            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .max_height(ui.available_height())
                .show(ui, |ui| {
                    for row in 0..self.num_tiers {
                        ui.add_sized(
                            [tier_size, tier_size],
                            Label::new(self.tiers.get(row as usize).unwrap()),
                        );
                    }
                });
        });
    }
}

fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}
