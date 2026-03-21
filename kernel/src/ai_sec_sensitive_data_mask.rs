extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct AISensitiveDataMask {
    sensitive_data: Vec<String>,
}

impl AISensitiveDataMask {
    pub fn new() -> Self {
        AISensitiveDataMask {
            sensitive_data: Vec::new(),
        }
    }

    pub fn add_sensitive_data(&mut self, data: &str) {
        self.sensitive_data.push(data.to_string());
    }

    pub fn remove_sensitive_data(&mut self, data: &str) -> bool {
        if let Some(index) = self.sensitive_data.iter().position(|d| d == data) {
            self.sensitive_data.remove(index);
            true
        } else {
            false
        }
    }

    pub fn contains_sensitive_data(&self, data: &str) -> bool {
        self.sensitive_data.contains(&data.to_string())
    }

    pub fn mask_data(&self, input: &str) -> String {
        let mut masked_input = input.to_string();
        for sensitive in &self.sensitive_data {
            if masked_input.contains(sensitive) {
                let mask_char = '*';
                let mask_len = sensitive.len();
                let start_index = masked_input.find(sensitive).unwrap();
                let end_index = start_index + mask_len;
                masked_input.replace_range(start_index..end_index, &mask_char.repeat(mask_len));
            }
        }
        masked_input
    }

    pub fn list_sensitive_data(&self) -> Vec<String> {
        self.sensitive_data.clone()
    }
}
