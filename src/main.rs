#[macro_use]
extern crate clap;

mod event;
mod selection;
mod loader;
mod args;
mod tui_mode;
mod background_mode;
mod draw;

use std::io;

use loader::{load_file, Word};
use args::{parse_arguments, Mode};
use tui_mode::{tui_routine};
use background_mode::{background_routine};

fn main() -> Result<(), io::Error> {
    // Read yaml file
    let categories = load_file("LSF.yaml");
    let all_words = categories.iter()
                              .cloned()
                              .flat_map(|c| c.words)
                              .collect::<Vec<Word>>();

    // Retrieve arguments
    let arguments = parse_arguments();
    match arguments.mode {
        Mode::TUI => tui_routine(categories, all_words),
        Mode::Background(sec) => background_routine(sec, all_words, arguments.description),
    }
}
