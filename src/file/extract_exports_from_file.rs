use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

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
