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

pub fn extract_exports_from_file(path: &Path, export_re: &regex::Regex) -> io::Result<Vec<String>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let exports: Vec<String> = export_re
        .captures_iter(&content)
        .map(|cap| cap[1].to_string())
        .collect();
    Ok(exports)
}
