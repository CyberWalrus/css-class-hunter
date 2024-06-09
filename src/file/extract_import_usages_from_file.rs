use pathdiff::diff_paths;
use regex::Regex;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use crate::app_config::get_config;

use super::resolve_import_alias::resolve_import_alias;

pub fn extract_import_usages_from_file(
    path: &Path,
    import_re: &Regex,
    tsconfig_paths: &HashMap<String, Vec<String>>,
) -> io::Result<HashMap<String, Vec<String>>> {
    // Открытие файла с тремя попытками
    let mut file = File::open(path)?;
    let mut content = String::new();

    let mut attempts = 3;
    while attempts > 0 {
        match file.read_to_string(&mut content) {
            Ok(_) => break,
            Err(e) if attempts > 1 => {
                eprintln!(
                    "Ошибка при чтении файла {:?}, попытка {}; ошибка: {}",
                    path,
                    4 - attempts,
                    e
                );
                attempts -= 1;
                sleep(Duration::from_millis(100));
            }
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "Не удалось прочитать файл {:?} после 3 попыток; ошибка: {}",
                        path, e
                    ),
                ));
            }
        }
    }

    let mut imports_map: HashMap<String, Vec<String>> = HashMap::new();

    // Найти все импорты .scss файлов
    for cap in import_re.captures_iter(&content) {
        let current_dir = current_dir().unwrap();
        let config = get_config().read().unwrap();

        // Здесь мы предполагаем, что проект запускается из корневой директории проекта,
        // поэтому текущая директория и будет корнем проекта.
        let project_root = current_dir
            .join(&config.folder_app_path)
            .canonicalize()
            .unwrap();

        let import_path = cap[2].to_string();

        // Преобразовать относительный путь импорта в полный путь
        // Попробовать использовать алиас для преобразования пути импорта
        let full_import_path = if let Some(resolved_path) =
            resolve_import_alias(tsconfig_paths, &import_path, &project_root)
        {
            resolved_path
        } else {
            path.parent()
                .unwrap()
                .join(&import_path)
                .canonicalize()
                .unwrap()
        };

        // Получить относительный путь (относительно project_root)
        let relative_import_path = diff_paths(&full_import_path, &project_root)
            .ok_or("Failed to obtain relative path")
            .unwrap();

        // Преобразовать относительный путь в строку
        let import_key = relative_import_path.to_string_lossy().to_string();

        let import_name = cap[1].to_string();

        // Найти использования import_name в коде
        let usage_re = Regex::new(&format!(r"{}\.([a-zA-Z0-9_-]+)", import_name)).unwrap();
        let mut usage_classes = Vec::new();

        for usage_cap in usage_re.captures_iter(&content) {
            usage_classes.push(usage_cap[1].to_string());
        }

        imports_map.insert(import_key, usage_classes);
    }

    Ok(imports_map)
}
