use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;

pub enum Event<I> {
    Input(I),
    Tick,
}

pub struct Events {
    pub tx: mpsc::Sender<Event<Key>>,
    pub rx: mpsc::Receiver<Event<Key>>,
}

impl Events {
    pub fn new(refresh_rate: u64) -> Self {
        let refresh_duration = Duration::from_millis(refresh_rate);
        let (tx, rx) = mpsc::channel();

        let _tick = {
            let tx = tx.clone();
            thread::spawn(move || {
                let mut _t = 0;
                loop {
                    tx.send(Event::Tick).unwrap_or_default();
                    _t += refresh_rate;
                    thread::sleep(refresh_duration);
                }
            })
        };

        let _input = {
            let tx = tx.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.keys() {
                    if let Ok(key) = evt {
                        tx.send(Event::Input(key)).unwrap_or_default();
                        if key == Key::Char('q') {
                            return;
                        };
                    };
                }
            })
        };

        Self { tx, rx }
    }
}
