extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraDnsOverHttps {
    server_url: String,
    query_cache: Vec<(String, String)>,
}

impl AuraDnsOverHttps {
    pub fn new(server_url: &str) -> Self {
        AuraDnsOverHttps {
            server_url: String::from(server_url),
            query_cache: Vec::new(),
        }
    }

    pub fn set_server_url(&mut self, server_url: &str) {
        self.server_url = String::from(server_url);
    }

    pub fn get_server_url(&self) -> &String {
        &self.server_url
    }

    pub fn add_to_cache(&mut self, query: &str, response: &str) {
        self.query_cache.push((String::from(query), String::from(response)));
    }

    pub fn clear_cache(&mut self) {
        self.query_cache.clear();
    }

    pub fn resolve(&self, query: &str) -> Option<&String> {
        for (q, r) in &self.query_cache {
            if q == query {
                return Some(r);
            }
        }
        None
    }
}
