use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

pub fn write_exports_to_file<W: Write>(
    output: &mut W,
    exports_map: Arc<Mutex<HashMap<String, Vec<String>>>>,
) -> io::Result<()> {
    let exports_map = exports_map.lock().unwrap();

    for (relative_path, exports) in exports_map.iter() {
        writeln!(output, "{}:", relative_path)?;
        for export in exports {
            writeln!(output, "  {}", export)?;
        }
    }
    Ok(())
}
