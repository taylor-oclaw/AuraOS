extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn asf_adapter_openapi_init() {
    // Initialization logic for the module
}

pub extern "C" fn asf_adapter_openapi_exit() {
    // Cleanup logic for the module
}

pub struct OpenAPIAdapter {
    base_url: String,
    api_key: String,
    endpoints: Vec<String>,
}

impl OpenAPIAdapter {
    pub fn new(base_url: &str, api_key: &str) -> Self {
        OpenAPIAdapter {
            base_url: String::from(base_url),
            api_key: String::from(api_key),
            endpoints: Vec::new(),
        }
    }

    pub fn add_endpoint(&mut self, endpoint: &str) {
        self.endpoints.push(String::from(endpoint));
    }

    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }

    pub fn get_api_key(&self) -> &str {
        &self.api_key
    }

    pub fn list_endpoints(&self) -> &[String] {
        &self.endpoints
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openapi_adapter() {
        let mut adapter = OpenAPIAdapter::new("https://api.example.com", "secret_key");
        assert_eq!(adapter.get_base_url(), "https://api.example.com");
        assert_eq!(adapter.get_api_key(), "secret_key");

        adapter.add_endpoint("/users");
        adapter.add_endpoint("/posts");
        assert_eq!(adapter.list_endpoints().len(), 2);
        assert_eq!(adapter.list_endpoints()[0], "/users");
        assert_eq!(adapter.list_endpoints()[1], "/posts");
    }
}
