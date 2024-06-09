use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub fn extract_exports_from_file(path: &Path) -> io::Result<Vec<String>> {
    // Открываем файл и читаем его содержимое в строку
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // HashSet для хранения уникальных имен экспортируемых сущностей
    let mut exports = HashSet::new();

    // Регулярное выражение для поиска экспортируемых классов, интерфейсов, констант, функций, типов и перечислений
    let export_re =
        Regex::new(r"export\s+(?:declare\s+)?(?:class|interface|const|function|type|enum)\s+(\w+)")
            .unwrap();
    // Регулярное выражение для поиска типов объектов
    let type_re = Regex::new(r"type\s+\w+\s*=\s*\{\s*([^}]+)\s*\}").unwrap();
    // Регулярное выражение для извлечения имен полей объектов
    let inner_re = Regex::new(r"(\w+):\s*\w+;?").unwrap();

    // Имена, которые нужно игнорировать
    let ignore_patterns = vec!["Styles", "ClassNames"];

    // Поиск экспортов по основному регулярному выражению
    for cap in export_re.captures_iter(&content) {
        let export_name = cap.get(1).unwrap().as_str();

        // Пропускаем игнорируемые имена
        if ignore_patterns.contains(&export_name) {
            continue;
        }

        // Добавляем найденное имя в HashSet
        exports.insert(export_name.to_string());
    }

    // Поиск типов объектов и извлечение их полей
    for cap in type_re.captures_iter(&content) {
        let fields_str = match cap.get(1) {
            Some(fields_str) => fields_str,
            None => continue,
        };

        for inner_cap in inner_re.captures_iter(fields_str.as_str()) {
            let field_name = match inner_cap.get(1) {
                Some(field_name) => field_name,
                None => continue,
            };

            exports.insert(field_name.as_str().to_string());
        }
    }

    // Преобразуем HashSet в вектор и возвращаем результат
    Ok(exports.into_iter().collect())
}
