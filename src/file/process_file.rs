use std::io::{self};
use std::path::Path;

pub fn process_file(path: &Path, root: &Path) -> io::Result<Option<String>> {
    if let Some(file_name) = path.to_str() {
        if file_name.ends_with(".d.ts") {
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
