use clap::App;

pub enum Mode {
    TUI,
    Background(u64),
    Video,
}

pub struct Arguments {
    pub mode: Mode,
    pub yaml: String,
    pub video_word: String,
    pub description: bool,
}

pub fn parse_arguments() -> Arguments {
    // arguments
    let cli_yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(cli_yaml).get_matches();
    let lsf_yaml = matches.value_of("yaml").unwrap_or("LSF.yaml");
    let description = matches.is_present("description");
    let video_word = matches.value_of("video").unwrap_or("bonjour");

    let mode = if matches.is_present("background") {
        let str_seconds = matches.value_of("background").unwrap_or("30");
        let int_seconds = str_seconds.parse::<u64>().unwrap();
        Mode::Background(int_seconds)
    } else if matches.is_present("video") {
        Mode::Video
    }else {
        Mode::TUI
    };

    Arguments {
        mode,
        yaml: lsf_yaml.to_string(),
        video_word: video_word.to_string(),
        description,
    }
}
