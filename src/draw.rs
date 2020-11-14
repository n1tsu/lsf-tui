use std::convert::TryFrom;

use std::io;
use std::time::{Duration};

// tui
use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders, List, Text, Paragraph, Tabs, Gauge};
use tui::layout::{Layout, Constraint, Direction, Alignment};
use tui::style::{Style, Color, Modifier};
use tui::symbols::DOT;
use tui::Terminal;

// local modules
use crate::selection::Selection;
use crate::loader::{Categorie, Word};

// Number of words to learn for a session in tab 'Learn'
pub static WORDS_LEARN_SIZE: usize = 20;

// Draw dictionary tab
// Terminal type is ugly af :)
pub fn draw_dictionary(terminal : &mut Terminal<TermionBackend<termion::raw::RawTerminal<io::Stdout>>>, states: &mut Selection, categories: &Vec<Categorie>) {
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
            .block(Block::default().title("Information").borders(Borders::ALL))
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
    }).unwrap();
}

pub fn draw_learn(terminal : &mut Terminal<TermionBackend<termion::raw::RawTerminal<io::Stdout>>>, words_set: &Vec<&Word>, time: &Duration, word_index: &usize, help: &mut bool) {
    terminal.draw(|mut f| {
        // Create vertical chunks
        let vert_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(4)].as_ref())
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
            .block(Block::default().title("Word").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center)
            .wrap(true);

        // Display of the time since the beginning
        let seconds = time.as_secs();
        let millis = time.as_millis() / 100 % 10;
        let text = [Text::styled(format!("{:?}.{} seconds\n\n", seconds, millis), Style::default().fg(Color::Green).modifier(Modifier::BOLD))];
        let time_text = Paragraph::new(text.iter())
            .block(Block::default().title("Time").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center)
            .wrap(true);

        // Display the index of the word
        let text = [Text::styled(format!("{}/{}\n\n", *word_index + 1, WORDS_LEARN_SIZE), Style::default().fg(Color::Green).modifier(Modifier::BOLD))];
        let index_text = Paragraph::new(text.iter())
            .block(Block::default().title("Progression").borders(Borders::ALL))
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

        // Create progression bar
        let progression = u16::try_from((*word_index + 1) * 100).unwrap() / u16::try_from(WORDS_LEARN_SIZE).unwrap();
        let gauge = Gauge::default()
            .block(Block::default().title("Progression").borders(Borders::ALL))
            .style(Style::default().fg(Color::Yellow))
            .percent(progression);


        // Render index
        f.render_widget(index_text, chunks[0]);
        // Render time
        f.render_widget(time_text, chunks[1]);
        // Render information about the word
        f.render_widget(para, chunks[2]);
        // Render tabs
        f.render_widget(tabs, vert_chunks[0]);
        // Render progression bar
        f.render_widget(gauge, vert_chunks[2]);
    }).unwrap();
}