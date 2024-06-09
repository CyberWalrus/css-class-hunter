use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::file::extract_exports_from_file::extract_exports_from_file;
use crate::file::process_file::process_file;

pub fn visit_dirs(
    dir: &Path,
    exports_map: Arc<Mutex<HashMap<String, Vec<String>>>>,
    root: &Path,
) -> io::Result<()> {
    if dir.is_dir() {
        let entries: Vec<_> = fs::read_dir(dir)?.collect::<Result<Vec<_>, _>>()?;

        // Используем многопоточность для обработки файлов и директорий.
        entries.par_iter().try_for_each(|entry| {
            let path = entry.path();
            if path.is_dir() {
                // Рекурсивно вызываем себя для директорий
                visit_dirs(&path, Arc::clone(&exports_map), root)
            } else {
                // Обрабатываем файлы
                if let Some(relative_path) = process_file(&path, root)? {
                    let exports = extract_exports_from_file(&path)?;
                    // Нам нужно использовать мьютекс для доступа к общему HashMap в многопоточном контексте
                    let mut map = exports_map.lock().unwrap();
                    if let Some(stripped_path) = relative_path.strip_suffix(".d.ts") {
                        let key = stripped_path.to_string();

                        map.insert(key, exports);
                    }
                }
                Ok(())
            }
        })?;
    }
    Ok(())
}
