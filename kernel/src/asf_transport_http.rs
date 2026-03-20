extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct HttpTransport {
    base_url: String,
    headers: Vec<(String, String)>,
}

impl HttpTransport {
    pub fn new(base_url: &str) -> Self {
        HttpTransport {
            base_url: String::from(base_url),
            headers: Vec::new(),
        }
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.push((String::from(key), String::from(value)));
    }

    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }

    pub fn get_headers(&self) -> &Vec<(String, String)> {
        &self.headers
    }

    pub fn send_get_request(&self, endpoint: &str) -> Result<String, &'static str> {
        // Simulate sending a GET request
        let full_url = format!("{}{}", self.base_url, endpoint);
        Ok(full_url)
    }

    pub fn send_post_request(&self, endpoint: &str, body: &str) -> Result<String, &'static str> {
        // Simulate sending a POST request
        let full_url = format!("{}{}", self.base_url, endpoint);
        Ok(format!("POST to {} with body {}", full_url, body))
    }
}
