extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Http2Handler {
    headers: Vec<(String, String)>,
    body: Option<Vec<u8>>,
}

impl Http2Handler {
    pub fn new() -> Self {
        Http2Handler {
            headers: Vec::new(),
            body: None,
        }
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        let key_str = String::from(key);
        let value_str = String::from(value);
        self.headers.push((key_str, value_str));
    }

    pub fn get_headers(&self) -> &Vec<(String, String)> {
        &self.headers
    }

    pub fn set_body(&mut self, body: Vec<u8>) {
        self.body = Some(body);
    }

    pub fn get_body(&self) -> Option<&Vec<u8>> {
        self.body.as_ref()
    }

    pub fn clear_headers(&mut self) {
        self.headers.clear();
    }
}
