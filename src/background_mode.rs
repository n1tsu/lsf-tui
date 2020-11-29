use notify_rust::Notification;
use rand::prelude::*;
use std::io;
use std::{thread, time};

use crate::loader::Word;

pub fn background_routine(
    sec: u64,
    all_words: Vec<Word>,
    description: bool,
) -> Result<(), io::Error> {
    let gap_time = time::Duration::from_secs(sec);
    let mut count = 0;

    loop {
        let random_number = rand::thread_rng().gen_range(0, all_words.len());
        let word = &all_words[random_number];
        let string_count = count.to_string();

        println!(
            "{} :
                 \r   Word        : {}
                 \r   Description : {}
                 \r   Link        : {}",
            string_count, word.name, word.description, word.link
        );

        let mut notif = Notification::new();
        notif.summary(&word.name).appname("lsf-tui");

        if description {
            notif.body(&word.description);
        }

        notif.show().unwrap();
        thread::sleep(gap_time);
        count += 1;
    }
}
