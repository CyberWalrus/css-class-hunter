mod app_config;
mod export_processing;
mod file_app_processing;
mod file_processing;
mod find_missing_entires;
mod ts_config;
mod visitor;
mod visitor_app;

use export_processing::write_exports_to_file;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use ts_config::get_paths_ts_config;
use visitor::visit_dirs;
use visitor_app::visit_app_dirs;

use find_missing_entires::find_missing_entires;

use app_config::init_config;

use crate::app_config::get_config;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <config_file>", args[0]);
        std::process::exit(1);
    }

    let config_file_path = &args[1];
    if let Err(e) = init_config(config_file_path) {
        eprintln!("Failed to initialize configuration: {e}");
        std::process::exit(1);
    }

    let config = get_config().read().unwrap();

    let folder_type_path = &config.folder_path;
    let folder_app_path = &config.folder_app_path;
    let output_file_path = &config.output_file;
    let output_file_app_path = &config.output_app_file;
    let tsconfig_file_path = &config.tsconfig_file;
    let missing_entries_path = &config.missing_entries_file;

    let start_time = Instant::now();
    let mut output_file = File::create(output_file_path)?;
    let mut output_file_app = File::create(output_file_app_path)?;
    let mut missing_entries_file = File::create(missing_entries_path)?;
    let exports_map: HashMap<String, Vec<String>> = HashMap::new();
    let exports_map = Arc::new(Mutex::new(exports_map));

    let import_map: HashMap<String, Vec<String>> = HashMap::new();
    let import_map = Arc::new(Mutex::new(import_map));
    let export_re =
        Regex::new(r"export\s+(?:declare\s+)?(?:class|interface|const|function|type|enum)\s+(\w+)")
            .unwrap();
    let import_re =
        regex::Regex::new(r#"\bimport\s+(.*)\s+from\s+['"`]([^'"`]+\.scss)['"`];?"#).unwrap();

    let tsconfig_paths = get_paths_ts_config(tsconfig_file_path)?;
    visit_dirs(
        std::path::Path::new(folder_type_path),
        Arc::clone(&exports_map),
        &export_re,
        std::path::Path::new(folder_type_path),
    )?;

    visit_app_dirs(
        std::path::Path::new(folder_app_path),
        Arc::clone(&import_map),
        &import_re,
        std::path::Path::new(folder_app_path),
        &tsconfig_paths,
    )?;
    let exports_map_clone = Arc::clone(&exports_map);
    let import_map_clone = Arc::clone(&import_map);

    let missing_imports = find_missing_entires(exports_map_clone, import_map_clone);

    write_exports_to_file(&mut output_file, Arc::clone(&exports_map))?;
    write_exports_to_file(&mut output_file_app, Arc::clone(&import_map))?;
    write_exports_to_file(&mut missing_entries_file, missing_imports)?;

    let duration = start_time.elapsed();
    eprintln!("Execution time: {:?}", duration);

    Ok(())
}
