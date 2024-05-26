use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::fs::File;
use std::io::{self, BufReader};
use std::sync::RwLock;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub folder_path: String,
    pub folder_app_path: String,
    pub output_file: String,
    pub output_app_file: String,
    pub missing_entries_file: String,
    pub tsconfig_file: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            folder_path: "default_folder_path".to_string(),
            folder_app_path: "default_folder_app_path".to_string(),
            output_file: "default_output_file".to_string(),
            output_app_file: "default_output_app_file".to_string(),
            missing_entries_file: "default_missing_entries_file".to_string(),
            tsconfig_file: "default_tsconfig_file".to_string(),
        }
    }
}

static APP_CONFIG: OnceCell<RwLock<AppConfig>> = OnceCell::new();

fn load(file_path: &str) -> io::Result<AppConfig> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let config: Result<AppConfig, _> = serde_json::from_reader(reader);

    match config {
        Ok(cfg) => Ok(cfg),
        Err(_) => Ok(AppConfig::default()), // Возврат дефолтных значений в случае ошибки
    }
}

pub fn init_config(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = load(file_path)?;
    APP_CONFIG
        .set(RwLock::new(config))
        .map_err(|_| "Failed to set CONFIG".into())
}

pub fn get_config() -> &'static RwLock<AppConfig> {
    APP_CONFIG.get().expect("Config is not initialized")
}
