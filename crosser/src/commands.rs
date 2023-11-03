use crossterm::{queue, execute, style::Print};
use std::{io, thread, time::Duration};

pub(crate) enum Commands {
    SaveFile,
    LoadFile,
    Bad,
}

pub(crate) fn execute_command(command: Commands) {
    match command {
        Commands::SaveFile => {
            execute!(io::stdout(), Print("written")).unwrap();
            thread::sleep(Duration::from_millis(1000))
        }
        Commands::LoadFile => {
            let _ = run!(Print("written"));
            thread::sleep(Duration::from_millis(1000))
        }
        Commands::Bad => {
            execute!(io::stdout(), Print("written")).unwrap();
            thread::sleep(Duration::from_millis(1000))
        }
    }
}

impl Into<Commands> for String {
    fn into(self) -> Commands {
        match self.as_str() {
            "w\n" => Commands::SaveFile,
            "l\n" => Commands::LoadFile,
            _ => Commands::Bad,
        }
    }
}
