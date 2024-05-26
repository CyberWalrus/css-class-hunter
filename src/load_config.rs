use serde::Deserialize;
use std::io;
use std::{fs::File, io::BufReader};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub folder_path: String,
    pub folder_app_path: String,
    pub output_file: String,
    pub output_app_file: String,
    pub tsconfig_file: String,
}

pub fn load_config(file_path: &str) -> io::Result<Config> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader)?;

    Ok(config)
}
