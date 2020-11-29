use std::convert::TryFrom;

use std::io;
use std::time::Duration;

// tui
use tui::backend::TermionBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::symbols::DOT;
use tui::widgets::{Block, Borders, Gauge, List, Paragraph, Tabs, Clear, ListItem, Wrap};
use tui::text::{Span, Spans};
use tui::Terminal;

// local modules
use crate::loader::{Categorie, Word};
use crate::selection::Selection;

pub enum WordState {
    Confirmed,
    Passed,
    Current,
    Next,
}

// Number of words to learn for a session in tab 'Learn'
// pub static WORDS_LEARN_SIZE: usize = 20;

// Function from tui-rs example source
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

// Draw dictionary tab
// Terminal type is ugly af :)
pub fn draw_dictionary(
    terminal: &mut Terminal<TermionBackend<termion::raw::RawTerminal<io::Stdout>>>,
    states: &mut Selection,
    categories: &[Categorie],
) {
    terminal
        .draw(|mut f| {
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
                    ]
                    .as_ref(),
                )
                .split(vert_chunks[1]); // These chunks are in the second vertical chunk

            // Create list of categories
            let cat_items: Vec<ListItem> = categories.iter()
                .map(|i| {
                    let mut lines = vec![Spans::from(Span::raw(&i.name))];
                    ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
                })
                .collect();

            let l_cat = List::new(cat_items)
                .block(Block::default().title("Categories").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">>");

            // Create list of words from actual category
            let items: Vec<ListItem> = categories[states.get_categorie_index()]
                .words
                .iter()
                .map(|i| {
                    let mut lines = vec![Spans::from(Span::raw(&i.name))];
                    ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
                })
                .collect();

            let l_word = List::new(items)
                .block(Block::default().title("Mots").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">>");

            // Create information about the word
            // - Word
            // - How to do it in LSF
            // - Link to video
            let mut text = vec![Spans::from(Span::styled(
                format!(
                    "{}\n\n",
                    &categories[states.get_categorie_index()].words[states.get_word_index()]
                        .name
                ),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ))];
            text.push(Spans::from(Span::styled(
                format!(
                    "{}\n",
                    &categories[states.get_categorie_index()].words[states.get_word_index()]
                        .description
                ),
                Style::default().fg(Color::Red))));
            text.push(Spans::from(Span::styled(
                format!(
                    "{}\n",
                    &categories[states.get_categorie_index()].words[states.get_word_index()]
                        .link
                ),
                Style::default().fg(Color::Blue))));

            let para = Paragraph::new(text)
                .block(Block::default().title("Information").borders(Borders::ALL))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });

            // Create the tabs
            let titles = vec![
                Spans::from(vec![Span::styled("Dictionary", Style::default().fg(Color::Yellow))]),
                Spans::from(vec![Span::styled("Learn", Style::default().fg(Color::Green))])
            ];

            let tabs = Tabs::new(titles)
                .block(Block::default().title("Mode").borders(Borders::ALL))
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
        })
        .unwrap();
}

pub fn draw_learn(
    terminal: &mut Terminal<TermionBackend<termion::raw::RawTerminal<io::Stdout>>>,
    words_learn_set: &mut Vec<(&Word, WordState)>,
    states: &mut Selection,
    time: &Duration,
    help: &mut bool,
) {
    terminal
        .draw(|mut f| {
            // Create vertical chunks
            let vert_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(0),
                        Constraint::Length(4),
                    ]
                    .as_ref(),
                )
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
                    ]
                    .as_ref(),
                )
                .split(vert_chunks[1]); // These chunks are in the second vertical chunk

            let word_index = states.get_word_index();
            // Display of the word
            let text = Spans::from(vec![Span::styled(
                format!("{}\n\n", words_learn_set[word_index].0.name),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{}\n", words_learn_set[word_index].0.description),
                Style::default().fg(Color::Red),
            ),
            Span::styled(
                format!("{}\n", words_learn_set[word_index].0.link),
                Style::default().fg(Color::Blue),
            )
            ]);

            let para = Paragraph::new(text)
                .block(Block::default().title("Word").borders(Borders::ALL))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });

            // Display of the time since the beginning
            let seconds = time.as_secs();
            let millis = time.as_millis() / 100 % 10;
            let text = Spans::from(vec![Span::styled(
                format!("{:?}.{} seconds\n\n", seconds, millis),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            )]);
            let time_text = Paragraph::new(text)
                .block(Block::default().title("Time").borders(Borders::ALL))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });

            // Display the index of the word
            let text = Spans::from(vec![Span::styled(
                format!("{}/{}\n\n", word_index + 1, words_learn_set.len()),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            )]);

            let index_text = Paragraph::new(text)
                .block(Block::default().title("Progression").borders(Borders::ALL))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });

            // Create the tabs
            let titles = vec![
                Spans::from(vec![Span::styled("Dictionary", Style::default().fg(Color::Yellow))]),
                Spans::from(vec![Span::styled("Learn", Style::default().fg(Color::Green))])
            ];

            let tabs = Tabs::new(titles)
                .block(Block::default().title("Mode").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .select(0) // Select is hardcoded
                .divider(DOT);

            // Create progression bar
            let progression = u16::try_from((word_index + 1) * 100).unwrap()
                / u16::try_from(words_learn_set.len()).unwrap();
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

            let words: Vec<ListItem> = words_learn_set
                .iter()
                .map(|(word, status)| {
                    let s = match status {
                        WordState::Passed => Style::default().bg(Color::Red),
                        WordState::Confirmed => Style::default().bg(Color::Green),
                        WordState::Current => Style::default().fg(Color::Gray),
                        _ => Style::default(),
                    };
                    let mut lines = vec![Spans::from(Span::styled(
                        format!("{}", word.name), s
                    ))];
                    ListItem::new(lines)
                })
                .collect();

            let words_list = List::new(words)
                .block(Block::default().borders(Borders::ALL).title("Words"));

            f.render_widget(words_list, chunks[1]);

            // Render final pop-up
            if states.is_done() {
                let size = f.size();
                let block = Block::default().title("Done").borders(Borders::ALL);
                let area = centered_rect(60, 20, size);
                f.render_widget(Clear, area); //this clears out the background
                f.render_widget(block, area);
            }
        })
        .unwrap();
}
