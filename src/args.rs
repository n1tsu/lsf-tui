use clap::App;

pub enum Mode {
    TUI,
    Background(u64),
}

pub struct Arguments {
    pub mode: Mode,
    pub yaml: String,
}

pub fn parse_arguments() -> Arguments {
    // arguments
    let cli_yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(cli_yaml).get_matches();
    let lsf_yaml = matches.value_of("yaml").unwrap_or("LSF.yaml");

    let mode =
        if matches.is_present("background") {
            let str_seconds = matches.value_of("background").unwrap_or("30");
            let int_seconds = str_seconds.parse::<u64>().unwrap();
            Mode::Background(int_seconds)
        } else {
            Mode::TUI
        };

    Arguments {
        mode,
        yaml: lsf_yaml.to_string(),
    }
}
