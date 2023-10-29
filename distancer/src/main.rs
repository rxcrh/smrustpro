mod application;

use application::Application;
use application::Mode;
use eframe::{NativeOptions, run_native, Error};
use std::io;

fn main() -> Result<(), Error> {
    
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();

    let m = match buffer.as_str() {
        "mouse\n" => Mode::Mouse,
        _ => Mode::Furthest,
    };
    

    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    run_native(
        "Distancer",
        options,
        Box::new(|_cc| Box::new(Application { mode: m, ..Default::default()})),
    )
}

