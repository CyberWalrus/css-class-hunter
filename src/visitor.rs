use crate::file_processing::{extract_exports_from_file, process_file};
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub fn visit_dirs(
    dir: &Path,
    exports_map: Arc<Mutex<HashMap<String, Vec<String>>>>,
    export_re: &Regex,
    root: &Path,
) -> io::Result<()> {
    if dir.is_dir() {
        let entries: Vec<_> = fs::read_dir(dir)?.collect::<Result<Vec<_>, _>>()?;

        // Используем многопоточность для обработки файлов и директорий.
        entries.par_iter().try_for_each(|entry| {
            let path = entry.path();
            if path.is_dir() {
                // Рекурсивно вызываем себя для директорий
                visit_dirs(&path, Arc::clone(&exports_map), export_re, root)
            } else {
                // Обрабатываем файлы
                if let Some(relative_path) = process_file(&path, root)? {
                    let exports = extract_exports_from_file(&path, export_re)?;
                    // Нам нужно использовать мьютекс для доступа к общему HashMap в многопоточном контексте
                    let mut map = exports_map.lock().unwrap();
                    map.insert(relative_path, exports);
                }
                Ok(())
            }
        })?;
    }
    Ok(())
}
