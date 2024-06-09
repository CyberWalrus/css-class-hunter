use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use regex::Regex;
use std::collections::HashSet;

pub fn extract_exports_from_file(path: &Path) -> io::Result<Vec<String>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut exports = HashSet::new();

    let export_re =
        Regex::new(r"export\s+(?:declare\s+)?(?:class|interface|const|function|type|enum)\s+(\w+)")
            .unwrap();
    let type_re = Regex::new(r"type\s+\w+\s*=\s*\{\s*([^}]+)\s*\}").unwrap();
    let inner_re = Regex::new(r"(\w+):\s*\w+;?").unwrap();
    let ignore_patterns = vec!["Styles", "ClassNames"];

    // Обработка основного регулярного выражения
    for cap in export_re.captures_iter(&content) {
        let export_name = cap.get(1).unwrap().as_str();
        if ignore_patterns.contains(&export_name) {
            break;
        }
        if let Some(m) = cap.get(1) {
            exports.insert(m.as_str().to_string());
        }
    }

    for cap in type_re.captures_iter(&content) {
        if let Some(m) = cap.get(1) {
            for inner_cap in inner_re.captures_iter(m.as_str()) {
                if let Some(n) = inner_cap.get(1) {
                    exports.insert(n.as_str().to_string());
                }
            }
        }
    }

    // Преобразование HashSet обратно в Vec
    Ok(exports.into_iter().collect())
}
