use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

pub trait Exportable: Debug {
    fn write_exports(&self, output: &mut dyn Write) -> io::Result<()>;
}

impl Exportable for HashMap<String, Vec<String>> {
    fn write_exports(&self, output: &mut dyn Write) -> io::Result<()> {
        for (relative_path, exports) in self.iter() {
            writeln!(output, "{}:", relative_path)?;
            for export in exports {
                writeln!(output, "  {}", export)?;
            }
        }
        Ok(())
    }
}

impl Exportable for HashMap<String, HashMap<String, Vec<String>>> {
    fn write_exports(&self, output: &mut dyn Write) -> io::Result<()> {
        for (relative_path, inner_map) in self.iter() {
            writeln!(output, "{}:", relative_path)?;
            for (inner_key, exports) in inner_map.iter() {
                writeln!(output, "  {}:", inner_key)?;
                for export in exports {
                    writeln!(output, "    {}", export)?;
                }
            }
        }
        Ok(())
    }
}

pub fn write_exports_to_file<W: Write, E: Exportable>(
    output: &mut W,
    exports_map: Arc<Mutex<E>>,
) -> io::Result<()> {
    let exports_map = exports_map.lock().unwrap();
    exports_map.write_exports(output)
}
