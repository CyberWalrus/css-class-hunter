use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn find_missing_entires(
    exports_map: Arc<Mutex<HashMap<String, Vec<String>>>>,
    import_map: Arc<Mutex<HashMap<String, Vec<String>>>>,
) -> Arc<Mutex<HashMap<String, Vec<String>>>> {
    let exports_map_locked = exports_map.lock().unwrap();
    let import_map_locked = import_map.lock().unwrap();

    let mut missing_entries: HashMap<String, Vec<String>> = HashMap::new();

    for (key, export_values) in exports_map_locked.iter() {
        // Получаем значения для данного ключа из import_map
        if let Some(import_values) = import_map_locked.get(key) {
            let mut diff: Vec<String> = Vec::new();
            for export_value in export_values {
                if !import_values.contains(export_value) {
                    diff.push(export_value.clone());
                }
            }
            if !diff.is_empty() {
                missing_entries.insert(key.clone(), diff);
            }
        } else {
            // Если ключа нет в import_map, все значения export_values добавляем в missing_entries
            missing_entries.insert(key.clone(), export_values.clone());
        }
    }

    let missing_entries = Arc::new(Mutex::new(missing_entries));

    missing_entries
}
