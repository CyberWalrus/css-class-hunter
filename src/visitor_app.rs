use crate::file_app_processing::{extract_import_usages_from_file, process_app_file};
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub fn visit_app_dirs(
    dir: &Path,
    exports_map: Arc<Mutex<HashMap<String, Vec<String>>>>,
    import_re: &Regex,
    root: &Path,
    tsconfig_paths: &HashMap<String, Vec<String>>,
) -> io::Result<()> {
    if dir.is_dir() {
        let entries: Vec<_> = fs::read_dir(dir)?.collect::<Result<Vec<_>, _>>()?;

        // Используем многопоточность для обработки файлов и директорий.
        entries.par_iter().try_for_each(|entry| {
            let path = entry.path();
            if path.is_dir() {
                // Рекурсивно вызываем себя для директорий
                visit_app_dirs(
                    &path,
                    Arc::clone(&exports_map),
                    import_re,
                    root,
                    tsconfig_paths,
                )
            } else {
                // Обрабатываем файлы
                if let Some(_relative_path) = process_app_file(&path, root)? {
                    let imports =
                        extract_import_usages_from_file(&path, import_re, tsconfig_paths)?;
                    let mut map = exports_map.lock().unwrap();
                    for (import_path, usages) in imports {
                        let entry = map.entry(import_path).or_insert_with(Vec::new);
                        entry.extend(usages);
                    }
                }
                Ok(())
            }
        })?;
    }
    Ok(())
}
