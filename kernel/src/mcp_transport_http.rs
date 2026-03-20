extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mcp_transport_http_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mcp_transport_http_exit() {
    // Cleanup logic for the module
}

pub struct HttpClient {
    base_url: String,
    headers: Vec<(String, String)>,
}

impl HttpClient {
    pub fn new(base_url: &str) -> Self {
        HttpClient {
            base_url: String::from(base_url),
            headers: Vec::new(),
        }
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.push((String::from(key), String::from(value)));
    }

    pub fn get(&self, path: &str) -> Result<String, &'static str> {
        // Simulate a GET request
        Ok(format!("GET {}{}", self.base_url, path))
    }

    pub fn post(&self, path: &str, body: &str) -> Result<String, &'static str> {
        // Simulate a POST request
        Ok(format!("POST {}{} with body {}", self.base_url, path, body))
    }

    pub fn put(&self, path: &str, body: &str) -> Result<String, &'static str> {
        // Simulate a PUT request
        Ok(format!("PUT {}{} with body {}", self.base_url, path, body))
    }

    pub fn delete(&self, path: &str) -> Result<String, &'static str> {
        // Simulate a DELETE request
        Ok(format!("DELETE {}{}", self.base_url, path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_client() {
        let mut client = HttpClient::new("http://example.com");
        client.add_header("Content-Type", "application/json");

        assert_eq!(client.get("/api").unwrap(), "GET http://example.com/api");
        assert_eq!(
            client.post("/api", "{\"key\":\"value\"}").unwrap(),
            "POST http://example.com/api with body {\"key\":\"value\"}"
        );
        assert_eq!(
            client.put("/api", "{\"key\":\"value\"}").unwrap(),
            "PUT http://example.com/api with body {\"key\":\"value\"}"
        );
        assert_eq!(client.delete("/api").unwrap(), "DELETE http://example.com/api");
    }
}
