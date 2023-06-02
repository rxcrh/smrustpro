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
    tiers: Vec<Vec<PathBuf>>,
}

impl Default for Content {
    fn default() -> Self {
        Self {
            tiers: vec![vec![], vec![], vec![], vec![], vec![]],
        }
    }
}

impl Content {
    fn get_assets(&mut self) {
        self.tiers[0].clear();

        let path = Path::new("assets/");

        for image in path.read_dir().expect("read dir failed") {
            if let Ok(image) = image {
                self.tiers[0].push(image.path());
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
            let length_tiers = self.tiers.len();
            ui.horizontal(|ui| {
                ui.heading("Rustier");
                if ui.button("Load").clicked() {
                    self.get_assets();
                }
            });

            let id_source = "my_drag_and_drop_demo";
            let mut source_col_row = None;
            let mut drop_col = None;
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .max_height(ui.available_height())
                .show(ui, |ui| {
                    ui.columns(length_tiers, |uis| {
                        for (col_idx, column) in self.tiers.clone().into_iter().enumerate() {
                            let ui = &mut uis[col_idx];
                            let can_accept_what_is_being_dragged = true; // We accept anything being dragged (for now) ¯\_(ツ)_/¯
                            let response =
                                drop_target(ui, can_accept_what_is_being_dragged, |ui| {
                                    ui.set_min_size(vec2(64.0, 100.0));
                                    for (row_idx, item) in column.iter().enumerate() {
                                        let item_id =
                                            Id::new(id_source).with(col_idx).with(row_idx);
                                        drag_source(ui, item_id, |ui| {
                                            let texture = ui.ctx().load_texture(
                                                "texture",
                                                self.load_image_from_path(Path::new(item)).unwrap(),
                                                Default::default(),
                                            );

                                            let imagus =
                                                egui::Image::new(&texture, texture.size_vec2());

                                            ui.add(imagus.sense(Sense::click()));
                                        });

                                        if ui.memory(|mem| mem.is_being_dragged(item_id)) {
                                            source_col_row = Some((col_idx, row_idx));
                                        }
                                    }
                                })
                                .response;

                            let is_being_dragged = ui.memory(|mem| mem.is_anything_being_dragged());
                            if is_being_dragged
                                && can_accept_what_is_being_dragged
                                && response.hovered()
                            {
                                drop_col = Some(col_idx);
                            }
                        }
                    });
                });

            if let Some((source_col, source_row)) = source_col_row {
                if let Some(drop_col) = drop_col {
                    if ui.input(|i| i.pointer.any_released()) {
                        let item = self.tiers[source_col].remove(source_row);
                        self.tiers[drop_col].push(item);
                    }
                }
            }
            //ui.horizontal(|ui| {
            //    for entry in self.image_paths.iter() {
            //        let texture = ui.ctx().load_texture(
            //            "texture",
            //            self.load_image_from_path(Path::new(entry)).unwrap(),
            //            Default::default(),
            //        );

            //        ui.add(egui::Image::new(&texture, texture.size_vec2()));
            //    }
            //});

            //ScrollArea::vertical()
            //    .auto_shrink([false; 2])
            //    .max_height(ui.available_height())
            //    .show(ui, |ui| {
            //        let mut idx = 0;
            //        for tier in self.tiers.clone().iter() {
            //            ui.horizontal(|ui| {
            //                ui.add_sized([tier_size, tier_size], Label::new(tier.get(0).unwrap()));
            //                for _entry in 0..5 {
            //                    // [TODO] make it a list that can be added to
            //                    ui.label("o");
            //                }
            //                if ui.button("-").clicked() {
            //                    self.tiers.remove(idx);
            //                }
            //            });
            //            idx += 1;
            //        }

            //        ui.horizontal(|ui| {
            //            for entry in self.image_paths.iter() {
            //                let texture = ui.ctx().load_texture(
            //                    "texture",
            //                    self.load_image_from_path(Path::new(entry)).unwrap(),
            //                    Default::default(),
            //                );

            //                ui.add(egui::Image::new(&texture, texture.size_vec2()));
            //            }
            //        });
            //    });
        });
    }
}

pub fn drag_source(ui: &mut Ui, id: Id, body: impl FnOnce(&mut Ui)) {
    let is_being_dragged = ui.memory(|mem| mem.is_being_dragged(id));

    if !is_being_dragged {
        let response = ui.scope(body).response;

        // Check for drags:
        let response = ui.interact(response.rect, id, Sense::drag());
        if response.hovered() {
            ui.ctx().set_cursor_icon(CursorIcon::Grab);
        }
    } else {
        ui.ctx().set_cursor_icon(CursorIcon::Grabbing);

        // Paint the body to a new layer:
        let layer_id = LayerId::new(Order::Tooltip, id);
        let response = ui.with_layer_id(layer_id, body).response;

        // Now we move the visuals of the body to where the mouse is.
        // Normally you need to decide a location for a widget first,
        // because otherwise that widget cannot interact with the mouse.
        // However, a dragged component cannot be interacted with anyway
        // (anything with `Order::Tooltip` always gets an empty [`Response`])
        // So this is fine!

        if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
            let delta = pointer_pos - response.rect.center();
            ui.ctx().translate_layer(layer_id, delta);
        }
    }
}

pub fn drop_target<R>(
    ui: &mut Ui,
    can_accept_what_is_being_dragged: bool,
    body: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    let is_being_dragged = ui.memory(|mem| mem.is_anything_being_dragged());

    let margin = Vec2::splat(4.0);

    let outer_rect_bounds = ui.available_rect_before_wrap();
    let inner_rect = outer_rect_bounds.shrink2(margin);
    let where_to_put_background = ui.painter().add(Shape::Noop);
    let mut content_ui = ui.child_ui(inner_rect, *ui.layout());
    let ret = body(&mut content_ui);
    let outer_rect = Rect::from_min_max(outer_rect_bounds.min, content_ui.min_rect().max + margin);
    let (rect, response) = ui.allocate_at_least(outer_rect.size(), Sense::hover());

    let style = if is_being_dragged && can_accept_what_is_being_dragged && response.hovered() {
        ui.visuals().widgets.active
    } else {
        ui.visuals().widgets.inactive
    };

    let mut fill = style.bg_fill;
    let mut stroke = style.bg_stroke;
    if is_being_dragged && !can_accept_what_is_being_dragged {
        fill = ui.visuals().gray_out(fill);
        stroke.color = ui.visuals().gray_out(stroke.color);
    }

    ui.painter().set(
        where_to_put_background,
        epaint::RectShape {
            rounding: style.rounding,
            fill,
            stroke,
            rect,
        },
    );

    InnerResponse::new(ret, response)
}
