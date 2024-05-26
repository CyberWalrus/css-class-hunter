use pathdiff::diff_paths;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

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

pub fn extract_import_usages_from_file(
    path: &Path,
    import_re: &Regex,
) -> io::Result<HashMap<String, Vec<String>>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut imports_map: HashMap<String, Vec<String>> = HashMap::new();

    // Найти все импорты .scss файлов
    for cap in import_re.captures_iter(&content) {
        let import_path = cap[2].to_string();

        // Преобразовать относительный путь импорта в полный путь
        let full_import_path = path
            .parent()
            .unwrap()
            .join(&import_path)
            .canonicalize()
            .unwrap();

        // Определить корень проекта (предположим, что скрипт запускается из корневой директории проекта)
        let project_root = Path::new("/Users/andreypakhomov/pet/css-class-hunter/app")
            .canonicalize()
            .unwrap();

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
