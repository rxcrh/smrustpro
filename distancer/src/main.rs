mod application;

use eframe::{NativeOptions, run_native, Error};
use crate::application::Application;

fn main() -> Result<(), Error> {
    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    run_native(
        "Distancer",
        options,
        Box::new(|_cc| Box::<Application>::default()),
    )
}

