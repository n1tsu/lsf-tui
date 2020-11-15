use rand::prelude::*;
use std::io;
use notify_rust::Notification;
use std::{time, thread};

use crate::loader::Word;

pub fn background_routine(sec: u64, all_words: Vec<Word>) -> Result<(), io::Error> {
    let gap_time = time::Duration::from_secs(sec);
    let mut count = 0;

    loop {
        let random_number = rand::thread_rng().gen_range(0, all_words.len());
        let word = &all_words[random_number];
        let string_count = count.to_string();
        println!("{} : {}", string_count, word.name);
        Notification::new()
            .summary(&string_count[..])
            .body(&word.name)
            .appname("lsf-tui")
            .show().unwrap();
        thread::sleep(gap_time);
        count += 1;
    }
}
