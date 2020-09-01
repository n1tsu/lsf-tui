mod event;
mod selection;
mod loader;

use rand::prelude::*;

use std::io;
use std::time::{Duration, Instant};

// tui
use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders, List, Text, Paragraph, Tabs,};
use tui::layout::{Layout, Constraint, Direction, Alignment};
use tui::style::{Style, Color, Modifier};
use tui::symbols::DOT;
use tui::Terminal;

// termion
use termion::raw::IntoRawMode;
use termion::event::Key;

// local modules
use event::{Events, Event};
use selection::Selection;
use loader::{load_file, Categorie, Word};


// Number of words to learn for a session in tab 'Learn'
static WORDS_LEARN_SIZE: usize = 20;



fn main() -> Result<(), io::Error> {
    // Read yaml file
    let categories = load_file("LSF.yaml");
    let all_words = categories.iter()
                              .cloned()
                              .flat_map(|c| c.words)
                              .collect::<Vec<Word>>();

    // Initialize terminal
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    // Spawn new threads for events
    // Get a 'Tick' information every 200ms and get inputs
    let events = Events::new(200);

    // Initialize current selection and current tab variable
    let mut states = Selection::new(categories.len());
    let mut tab_index = 0;

    // Variable to determine if we just swapped between tabs
    // If swap is 0 we were on tab 0 on last loop and if swap
    // is 1 we were on tab 1 on last loop.
    let mut swap = 0;
    let mut begin = Instant::now();
    let mut words_set = vec![];
    let mut word_index = 0;
    let mut help = false;

    loop {
        // Call update function and quit if it return 'Stop'
        match update(&events, &mut tab_index, &mut states, &categories, &mut word_index, &mut help) {
            UpdateState::Stop => break,
            // Refresh the TUI widgets
            UpdateState::Continue => {
                if tab_index == 0 {
                    // Draw the dictionary mode
                    draw_dictionary(&mut terminal, &mut states, &categories);
                    swap = 0;
                } else if tab_index == 1 {
                    // Reset variable because we swap tab
                    if swap == 0 {
                        begin = Instant::now();
                        let mut rng = rand::thread_rng();
                        words_set = all_words.iter().choose_multiple(&mut rng,  WORDS_LEARN_SIZE);
                        word_index = 0;
                        help = false;
                    }
                    // Calculate time since swap
                    let now = Instant::now();
                    let time = now.duration_since(begin);

                    // Draw the learn mode
                    draw_learn(&mut terminal, &words_set, &time, &word_index, &mut help);
                    swap = 1;
                }
            },
        }
    }
    Ok(())
}

enum UpdateState {
    Stop,
    Continue,
}

fn update(events: &Events, tab_index: &mut usize, states: &mut Selection, categories: &Vec<Categorie>, word_index: &mut usize, help: &mut bool) -> UpdateState {
    // Try to receive an event, handle it if any, then just return
    match events.rx.try_recv() {
        // An event has been sent, let's handle it
        Ok(x) => {
            // If this event is an input, do some actions
            if let Event::Input(input) = x {
                match input {
                    // Quit
                    Key::Char('q') => {
                        return UpdateState::Stop
                    }
                    // Move selection
                    Key::Char('j') => {
                        states.down();
                    }
                    Key::Char('k') => {
                        states.up();
                    }
                    Key::Char('h') => {
                        states.focus_left();
                    }
                    Key::Char('l') => {
                        states.focus_right(categories[states.get_categorie_index()].words.len());
                    }
                    // Change tabs
                    Key::Char('1') => {
                        *tab_index = 0;
                    }
                    Key::Char('2') => {
                        *tab_index = 1;
                    }
                    // Change word index in learn
                    Key::Char('n') => {
                        *word_index += 1;
                        *help = false;
                    }
                    // Display help in learn
                    Key::Char('m') => {
                        *help = !*help;
                    }
                    _ => {}
                };
            };
            // We could had some action to handle 'Tick' event here
        }

        // There is no event, we do nothing
        // It might be a good idea to add a sleep here to IDLE the CPU
        _ => {},
    }
  UpdateState::Continue
}

// Draw dictionary tab
// Terminal type is ugly af :)
fn draw_dictionary(terminal : &mut Terminal<TermionBackend<termion::raw::RawTerminal<io::Stdout>>>, states: &mut Selection, categories: &Vec<Categorie>) {
    terminal.draw(|mut f| {
        // Create vertical chunks
        let vert_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(f.size());

        // Create horizontal chunks
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                ].as_ref()
            )
            .split(vert_chunks[1]); // These chunks are in the second vertical chunk

        // Create list of categories
        let cat_items = categories.iter().map(|i| Text::raw(&i.name));
        let l_cat = List::new(cat_items)
            .block(Block::default().title("Categories").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        // Create list of words from actual category
        let items = categories[states.get_categorie_index()].words.iter().map(|i| Text::raw(&i.name));
        let l_word = List::new(items)
            .block(Block::default().title("Mots").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        // Create information about the word
        // - Word
        // - How to do it in LSF
        // - Link to video
        let text = [
            Text::styled(format!("{}\n\n", &categories[states.get_categorie_index()].words[states.get_word_index()].name), Style::default().fg(Color::Green).modifier(Modifier::BOLD)),
            Text::styled(format!("{}\n", &categories[states.get_categorie_index()].words[states.get_word_index()].description), Style::default().fg(Color::Red)),
            Text::styled(format!("{}\n", &categories[states.get_categorie_index()].words[states.get_word_index()].link), Style::default().fg(Color::Blue))];
        let para = Paragraph::new(text.iter())
            .block(Block::default().title("Paragraph").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center)
            .wrap(true);

        // Create the tabs
        let tabs = Tabs::default()
            .block(Block::default().title("Mode").borders(Borders::ALL))
            .titles(&["Dictionary", "Learn"])
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .select(0) // Select is hardcoded
            .divider(DOT);

        // Render list of categories
        f.render_stateful_widget(l_cat, chunks[0], &mut states.categorie_state);
        // Render list of words
        f.render_stateful_widget(l_word, chunks[1], &mut states.word_state);
        // Render information about the word
        f.render_widget(para, chunks[2]);
        // Render tabs
        f.render_widget(tabs, vert_chunks[0]);
    });
}

fn draw_learn(terminal : &mut Terminal<TermionBackend<termion::raw::RawTerminal<io::Stdout>>>, words_set: &Vec<&Word>, time: &Duration, word_index: &usize, help: &mut bool) {
    terminal.draw(|mut f| {
        // Create vertical chunks
        let vert_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(f.size());

        // Create horizontal chunks
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                ].as_ref()
            )
            .split(vert_chunks[1]); // These chunks are in the second vertical chunk

        // Display of the word
        let mut text = vec![Text::styled(format!("{}\n\n", words_set[*word_index].name), Style::default().fg(Color::Green).modifier(Modifier::BOLD))];
        let description = Text::styled(format!("{}\n", words_set[*word_index].description), Style::default().fg(Color::Red));
        let link = Text::styled(format!("{}\n", words_set[*word_index].link), Style::default().fg(Color::Blue));


        if *help {
            text.push(description);
            text.push(link);
        }

        let para = Paragraph::new(text.iter())
            .block(Block::default().title("Paragraph").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center)
            .wrap(true);

        // Display of the time since the beginning
        let seconds = time.as_secs();
        let millis = time.as_millis() / 100 % 10;
        let text = [Text::styled(format!("{:?}.{} seconds\n\n", seconds, millis), Style::default().fg(Color::Green).modifier(Modifier::BOLD))];
        let time_text = Paragraph::new(text.iter())
            .block(Block::default().title("Paragraph").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center)
            .wrap(true);

        // Display the index of the word
        let text = [Text::styled(format!("{}/{}\n\n", *word_index, WORDS_LEARN_SIZE), Style::default().fg(Color::Green).modifier(Modifier::BOLD))];
        let index_text = Paragraph::new(text.iter())
            .block(Block::default().title("Paragraph").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center)
            .wrap(true);

        // Create the tabs
        let tabs = Tabs::default()
            .block(Block::default().title("Mode").borders(Borders::ALL))
            .titles(&["Dictionary", "Learn"])
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .select(1) // Select is hardcoded
            .divider(DOT);


        // Render index
        f.render_widget(index_text, chunks[0]);
        // Render time
        f.render_widget(time_text, chunks[1]);
        // Render information about the word
        f.render_widget(para, chunks[2]);
        // Render tabs
        f.render_widget(tabs, vert_chunks[0]);
    });
}
