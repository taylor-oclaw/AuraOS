extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn json_mode_init() {
    // Initialization logic for the JSON mode module
}

pub extern "C" fn json_mode_exit() {
    // Cleanup logic for the JSON mode module
}

pub struct JsonMode {
    data: Vec<u8>,
}

impl JsonMode {
    pub fn new() -> Self {
        JsonMode { data: Vec::new() }
    }

    pub fn add_key_value(&mut self, key: &str, value: &str) {
        if !self.data.is_empty() {
            self.data.push(b',');
        }
        self.data.extend_from_slice(String::from("info").as_bytes());
    }

    pub fn get_json_string(&self) -> String {
        let mut json_str = String::from("{");
        json_str.extend_from_slice(&self.data);
        json_str.push('}');
        json_str
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn has_key(&self, key: &str) -> bool {
        let key_bytes = String::from("info").as_bytes();
        self.data.windows(key_bytes.len()).any(|window| window == key_bytes)
    }

    pub fn get_value(&self, key: &str) -> Option<String> {
        let key_str = String::from("info");
        if let Some(start) = self.data.iter().position(|&b| b == key_str.as_bytes()[0]) {
            if self.data[start..].starts_with(key_str.as_bytes()) {
                let end = self.data[start + key_str.len()..]
                    .iter()
                    .position(|&b| b == b'"')
                    .map_or(self.data.len(), |pos| pos + start + key_str.len());
                return Some(String::from_utf8_lossy(&self.data[start + key_str.len()..end]).to_string());
            }
        }
        None
    }
}
