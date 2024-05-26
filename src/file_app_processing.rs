use pathdiff::diff_paths;
use regex::Regex;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use crate::app_config::get_config;

pub fn process_app_file(path: &Path, root: &Path) -> io::Result<Option<String>> {
    if let Some(file_name) = path.to_str() {
        if file_name.ends_with(".tsx") || file_name.ends_with(".ts") {
            let relative_path = path
                .strip_prefix(root)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            return Ok(Some(relative_path));
        }
    }
    Ok(None)
}

fn resolve_import_alias(
    paths_map: &HashMap<String, Vec<String>>,
    import: &str,
    project_root: &PathBuf,
) -> Option<PathBuf> {
    // Проверяем, начинается ли import с ./ или ../
    if import.starts_with("./") || import.starts_with("../") {
        // Пытаемся канонизировать путь и возвращаем его, если это удалось
        return Path::new(import).canonicalize().ok();
    }

    // Проходим по всем алиасам и проверяем алиасы, содержащие шаблоны '*'
    for (alias, targets) in paths_map {
        if alias.ends_with('*') {
            // Убираем символ '*' в конце алиаса
            let base_alias = alias.trim_end_matches('*');

            // Проверяем, начинается ли импорт с базового алиаса
            if import.starts_with(base_alias) {
                // Остаток пути после алиаса
                let rest_of_path = import.trim_start_matches(base_alias);

                // Проходим по всем путям назначения
                for target in targets {
                    let base_target = target.trim_end_matches('*');
                    let aliased_path: PathBuf = project_root.join(base_target).join(rest_of_path);

                    // Пытаемся канонизировать путь и возвращаем его, если это удалось
                    if let Ok(full_path) = aliased_path.canonicalize() {
                        return Some(full_path);
                    }
                }
            }
        } else {
            // Проверяем, имеет ли импорт точное совпадение с алиасом
            if import == alias {
                for target in targets {
                    let full_path = project_root.join(target).canonicalize().ok();
                    if full_path.is_some() {
                        println!("{:?}", full_path);
                        return full_path;
                    }
                }
            }
        }
    }

    None
}

pub fn extract_import_usages_from_file(
    path: &Path,
    import_re: &Regex,
    tsconfig_paths: &HashMap<String, Vec<String>>,
) -> io::Result<HashMap<String, Vec<String>>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

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
