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
        Ok(format!("GET {}{}", self.base_url, path))
    }

    pub fn post(&self, path: &str, body: &[u8]) -> Result<String, &'static str> {
        // Simulate a POST request
        let body_str = core::str::from_utf8(body).map_err(|_| "Invalid UTF-8")?;
        Ok(format!("POST {}{} with body {}", self.base_url, path, body_str))
    }

    pub fn put(&self, path: &str, body: &[u8]) -> Result<String, &'static str> {
        // Simulate a PUT request
        let body_str = core::str::from_utf8(body).map_err(|_| "Invalid UTF-8")?;
        Ok(format!("PUT {}{} with body {}", self.base_url, path, body_str))
    }

    pub fn delete(&self, path: &str) -> Result<String, &'static str> {
        // Simulate a DELETE request
        Ok(format!("DELETE {}{}", self.base_url, path))
    }

    pub fn head(&self, path: &str) -> Result<String, &'static str> {
        // Simulate a HEAD request
        Ok(format!("HEAD {}{}", self.base_url, path))
    }
}
