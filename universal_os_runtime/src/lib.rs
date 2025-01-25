use wasm_bindgen::prelude::*;
use std::collections::HashMap;

#[wasm_bindgen]
pub struct Runtime {
    files: HashMap<String, String>,
}

#[wasm_bindgen]
impl Runtime {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Runtime {
        Runtime {
            files: HashMap::new(),
        }
    }

    pub fn execute(&mut self, command: &str) -> String {
        let parts: Vec<&str> = command.split_whitespace().collect();
        match parts.as_slice() {
            ["print", message] => {
                let output = message.replace("\"", "");
                output
            }
            ["fs.create", path, content] => {
                self.files.insert(path.to_string(), content.replace("\"", ""));
                format!("File created at {}", path)
            }
            ["fs.read", path] => {
                if let Some(content) = self.files.get(*path) {
                    content.clone()
                } else {
                    format!("File {} not found", path)
                }
            }
            _ => "Unknown command".to_string(),
        }
    }
}
