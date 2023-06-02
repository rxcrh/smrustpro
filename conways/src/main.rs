use std::thread::sleep;
use std::time::Duration;
use std::process::Command;

// how long does one iteration take
const DAY: u64 = 1;

struct World {
    width: usize,
    height: usize,
}

impl Default for World {
    fn default() -> Self {
        Self { 
            width: 20,
            height: 20,
        }
    }
}

fn main() {
    game_loop();
}

fn game_loop() {
    let world = World::default(); 
    loop {
        Command::new("clear").status().unwrap(); 

        draw_world(&world);

        sleep(Duration::from_secs(DAY));
    }
}

fn draw_world(world: &World) {
    for _ in 0..world.height {
        println!("{}", "#".repeat(world.width));
    }
}
