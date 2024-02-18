use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn write_answer_to_json(file_path: &str, json_value: Value) {
    let path = Path::new(file_path);
    let mut file = File::create(path).expect("Failed to create file.");

    if let Err(e) = file.write_all(json_value.to_string().as_bytes()) {
        eprintln!("Failed to write to file: {}", e);
    }
}
