use std::path::{Path, PathBuf};

use eframe::egui;
use egui::*;
use image;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rustier",
        options,
        Box::new(|_cc| Box::<Content>::default()),
    )
}

struct Content {
    tiers: Vec<Vec<String>>,
    image_paths: Vec<PathBuf>,
}

impl Default for Content {
    fn default() -> Self {
        Self {tiers: vec![
            vec!["a".to_owned()],
            vec!["b".to_owned()],
            vec!["c".to_owned()],

        ],
        image_paths: vec![],
        }
    }
}

impl Content {
    fn get_assets(&mut self) {
        self.image_paths.clear();

        let path = Path::new("assets/");

        for image in path.read_dir().expect("read dir failed") {
            if let Ok(image) = image {
                self.image_paths.push(image.path());
            }
        }
    }

    fn load_image_from_path(
        &self,
        path: &std::path::Path,
    ) -> Result<egui::ColorImage, image::ImageError> {
        let image = image::io::Reader::open(path)?.decode()?;
        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        Ok(egui::ColorImage::from_rgba_unmultiplied(
            size,
            pixels.as_slice(),
        ))
    }
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let tier_size: f32 = 0.05 * ui.available_width();

            ui.horizontal(|ui| {
                ui.heading("Rustier");
                if ui.button("+").clicked() {
                    self.tiers.push(vec!["x".to_owned()]);
                }
                if ui.button("Load").clicked() {
                    self.get_assets();
                }
            });

            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .max_height(ui.available_height())
                .show(ui, |ui| {
                    for tier in self.tiers.clone().iter() {
                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [tier_size, tier_size],
                                Label::new(tier.get(0).unwrap()),
                            );
                            for _entry in 0..5 { //tier.iter() {
                                ui.label("o");
                            }
                            if ui.button("-").clicked() {
                                self.tiers.pop();
                            }
                        });
                    }

                    ui.horizontal(|ui| {
                        for entry in self.image_paths.iter() {
                            let texture = ui.ctx().load_texture(
                                "texture",
                                self.load_image_from_path(Path::new(entry)).unwrap(),
                                Default::default(),
                            );

                            ui.add(egui::Image::new(&texture, texture.size_vec2()));
                        }
                    });
                });
        });
    }
}
