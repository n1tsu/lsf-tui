mod event;
mod selection;

use std::io;
use std::io::prelude::*;
use std::fs::File;

use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders, List, ListState, Text, Paragraph, Tabs,};
use tui::layout::{Layout, Constraint, Direction, Alignment};
use tui::style::{Style, Color, Modifier};
use tui::symbols::DOT;

use termion::raw::IntoRawMode;
use termion::event::Key;

use yaml_rust::YamlLoader;

use event::{Events, Event};
use selection::Selection;



pub struct Categorie {
    pub name : String,
    pub words : Vec<Word>,
}

pub struct Word {
    pub name : String,
    pub description : String,
    pub link : String,
}

fn load_file(file: &str) -> Vec<Categorie> {
    let mut file = File::open(file).expect("Unable to open file");
    let mut contents = String::new();

    let mut res = Vec::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let doc = &docs[0];

    for i in doc["categories"].as_vec().unwrap() {
        let mut words = Vec::new();
        for y in i["mots"].as_vec().unwrap() {
            let word = Word {
                name : String::from(y["mot"].as_str().unwrap()),
                description : String::from(y["description"].as_str().unwrap()),
                link : String::from(y["lien"].as_str().unwrap()),
            };
            words.push(word);
        }
        let categorie = Categorie {
            name : String::from(i["categorie"].as_str().unwrap()),
            words : words,
        };
        res.push(categorie);
    }
    res
}

fn main() -> Result<(), io::Error> {

    let categories = load_file("LSF.yaml");

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    terminal.hide_cursor()?;

    let events = Events::new(200);
    let mut states = Selection::new(categories.len());
    let mut tab_index = 0;

    loop {
        match events.rx.try_recv() {
            Ok(x) => {
                if let Event::Input(input) = x {
                    match input {
                        Key::Char('q') => {
                            return Ok(())
                        }
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
                        Key::Char('1') => {
                            tab_index = 0;
                        }
                        Key::Char('2') => {
                            tab_index = 1;
                        }
                        _ => {}
                    };
                };

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
            }
            _ => {},
        };
    }
}
