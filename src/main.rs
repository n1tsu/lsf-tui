#[macro_use]
extern crate clap;

mod args;
mod background_mode;
mod draw;
mod event;
mod loader;
mod selection;
mod tui_mode;

use std::io;

use args::{parse_arguments, Mode};
use background_mode::background_routine;
use loader::{load_file, Word};
use tui_mode::tui_routine;

fn main() -> Result<(), io::Error> {
    // Read yaml file
    let categories = load_file("LSF.yaml");
    let all_words = categories
        .iter()
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
