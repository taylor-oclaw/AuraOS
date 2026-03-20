extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct HttpClient {
    base_url: String,
}

impl HttpClient {
    pub fn new(base_url: &str) -> Self {
        HttpClient {
            base_url: String::from(base_url),
        }
    }

    pub fn get(&self, path: &str) -> Result<String, &'static str> {
        // Simulate a GET request
        Ok(String::from("info"))
    }

    pub fn post(&self, path: &str, body: &[u8]) -> Result<String, &'static str> {
        // Simulate a POST request
        let body_str = core::str::from_utf8(body).map_err(|_| "Invalid UTF-8")?;
        Ok(String::from("info"))
    }

    pub fn put(&self, path: &str, body: &[u8]) -> Result<String, &'static str> {
        // Simulate a PUT request
        let body_str = core::str::from_utf8(body).map_err(|_| "Invalid UTF-8")?;
        Ok(String::from("info"))
    }

    pub fn delete(&self, path: &str) -> Result<String, &'static str> {
        // Simulate a DELETE request
        Ok(String::from("info"))
    }

    pub fn head(&self, path: &str) -> Result<String, &'static str> {
        // Simulate a HEAD request
        Ok(String::from("info"))
    }
}
