mod config;
mod export_processing;
mod file_app_processing;
mod file_processing;
mod load_config;
mod visitor;
mod visitor_app;

use config::extract_paths_from_file;
use export_processing::write_exports_to_file;
use load_config::load_config;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use visitor::visit_dirs;
use visitor_app::visit_app_dirs;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <config_file>", args[0]);
        std::process::exit(1);
    }

    let config_file_path = &args[1];
    let config = match load_config(config_file_path) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error reading config file: {}", err);
            std::process::exit(1);
        }
    };

    let folder_type_path = &config.folder_path;
    let folder_app_path = &config.folder_app_path;
    let output_file_path = &config.output_file;
    let output_file_app_path = &config.output_app_file;
    let tsconfig_file_path = &config.tsconfig_file;

    let start_time = Instant::now();
    let mut output_file = File::create(output_file_path)?;
    let mut output_file_app = File::create(output_file_app_path)?;
    let exports_map: HashMap<String, Vec<String>> = HashMap::new();
    let exports_map = Arc::new(Mutex::new(exports_map));

    let import_map: HashMap<String, Vec<String>> = HashMap::new();
    let import_map = Arc::new(Mutex::new(import_map));
    let export_re =
        Regex::new(r"export\s+(?:class|interface|const|function|type|enum)\s+(\w+)").unwrap();

    extract_paths_from_file(tsconfig_file_path)?;
    visit_dirs(
        std::path::Path::new(folder_type_path),
        Arc::clone(&exports_map), // Используем Arc::clone для клонирования `exports_map`
        &export_re,
        std::path::Path::new(folder_type_path),
    )?;

    visit_app_dirs(
        std::path::Path::new(folder_app_path),
        Arc::clone(&import_map), // Используем Arc::clone для клонирования `exports_map`
        &export_re,
        std::path::Path::new(folder_app_path),
    )?;

    write_exports_to_file(&mut output_file, Arc::clone(&exports_map))?;
    write_exports_to_file(&mut output_file_app, Arc::clone(&import_map))?;

    let duration = start_time.elapsed();
    eprintln!("Execution time: {:?}", duration);

    Ok(())
}
