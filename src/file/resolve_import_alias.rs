use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub fn resolve_import_alias(
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
