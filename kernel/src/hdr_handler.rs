extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[allow(non_camel_case_types)]
pub struct hdr_handler {
    headers: Vec<(String, String)>,
}

impl hdr_handler {
    pub fn new() -> Self {
        hdr_handler {
            headers: Vec::new(),
        }
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        let key_str = String::from(key);
        let value_str = String::from(value);
        self.headers.push((key_str, value_str));
    }

    pub fn get_header(&self, key: &str) -> Option<&String> {
        for (k, v) in &self.headers {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn remove_header(&mut self, key: &str) {
        self.headers.retain(|(k, _)| k != key);
    }

    pub fn has_header(&self, key: &str) -> bool {
        for (k, _) in &self.headers {
            if k == key {
                return true;
            }
        }
        false
    }

    pub fn list_headers(&self) -> Vec<(String, String)> {
        self.headers.clone()
    }
}
