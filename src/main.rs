mod event;
mod selection;
mod loader;

use std::io;

use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders, List, Text, Paragraph, Tabs,};
use tui::layout::{Layout, Constraint, Direction, Alignment};
use tui::style::{Style, Color, Modifier};
use tui::symbols::DOT;
use tui::Terminal;


use termion::raw::IntoRawMode;
use termion::event::Key;

use event::{Events, Event};
use selection::Selection;
use loader::{load_file, Categorie};

fn main() -> Result<(), io::Error> {
    // Read yaml file
    let categories = load_file("LSF.yaml");

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

    loop {
        // Call update function and quit if it return 'Stop'
        match update(&events, &mut tab_index, &mut states, &categories) {
            UpdateState::Stop => break,
            // Refresh the TUI widgets
            UpdateState::Continue => {
                if tab_index == 0 {
                    draw_dictionary(&mut terminal, &mut states, &categories);
                } else if tab_index == 1 {

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

fn update(events: &Events, tab_index: &mut usize, states: &mut Selection, categories: &Vec<Categorie>) -> UpdateState {
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
                    _ => {}
                };
            };
            // We could had some action to handle 'Tick' event here

            // We refresh the widgets here
            /*
             terminal.draw(|mut f| {
                 let stdin = io::stdin();

                 let vert_chunks = Layout::default()
                     .direction(Direction::Vertical)
                     .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                     .split(f.size());

                 let chunks = Layout::default()
                     .direction(Direction::Horizontal)
                     .margin(1)
                     .constraints(
                         [
                             Constraint::Percentage(20),
                             Constraint::Percentage(20),
                             Constraint::Percentage(60),
                         ].as_ref()
                     )
                     .split(vert_chunks[1]);

                 let cat_items = categories.iter().map(|i| Text::raw(&i.name));
                 let l = List::new(cat_items)
                     .block(Block::default().title("Categories").borders(Borders::ALL))
                     .style(Style::default().fg(Color::White))
                     .highlight_style(Style::default().modifier(Modifier::ITALIC))
                     .highlight_symbol(">>");
                 f.render_stateful_widget(l, chunks[0], &mut states.categorie_state);

                 let items = categories[states.get_categorie_index()].words.iter().map(|i| Text::raw(&i.name));
                 let l = List::new(items)
                     .block(Block::default().title("Mots").borders(Borders::ALL))
                     .style(Style::default().fg(Color::White))
                     .highlight_style(Style::default().modifier(Modifier::ITALIC))
                     .highlight_symbol(">>");
                 f.render_stateful_widget(l, chunks[1], &mut states.word_state);

                 let text = [
                     Text::styled(format!("{}\n\n", &categories[states.get_categorie_index()].words[states.get_word_index()].name), Style::default().fg(Color::Green).modifier(Modifier::BOLD)),
                     Text::styled(format!("{}\n", &categories[states.get_categorie_index()].words[states.get_word_index()].description), Style::default().fg(Color::Red)),
                     Text::styled(format!("{}\n", &categories[states.get_categorie_index()].words[states.get_word_index()].link), Style::default().fg(Color::Blue))];
                 let para = Paragraph::new(text.iter())
                     .block(Block::default().title("Paragraph").borders(Borders::ALL))
                     .style(Style::default().fg(Color::White).bg(Color::Black))
                     .alignment(Alignment::Center)
                     .wrap(true);
                 f.render_widget(para, chunks[2]);

                 // Tabs
                 let tabs = Tabs::default()
                     .block(Block::default().title("Mode").borders(Borders::ALL))
                     .titles(&["Dictionary", "Learn"])
                     .style(Style::default().fg(Color::White))
                     .highlight_style(Style::default().fg(Color::Yellow))
                     .select(tab_index)
                     .divider(DOT);
                 f.render_widget(tabs, vert_chunks[0]);
             });
        */
        }

        // There is no event
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
            .margin(1)
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
