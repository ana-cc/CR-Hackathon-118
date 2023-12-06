use clap::App;
use clap::Arg;

pub struct AppConfig {
    pub file: String,
    pub filename: String,
}

impl AppConfig {
    pub fn from_clap() -> std::result::Result<Self, String> {
        let matches = App::new("qlog-baby-dancer")
        .version("v0.1.0")
        .about("Insert description here")
        .arg(
            Arg::with_name("LOG FILE")
                .help("Sets the input log file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

        let file = matches.value_of("LOG FILE").unwrap();
        let filename = std::path::Path::new(file)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        let config = Self {
            file: file.to_string(),
            filename: filename.to_string(),
        };

        Ok(config)
    }
}