extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentResponseCache {
    cache: Vec<(String, String)>,
}

impl AgentResponseCache {
    pub fn new() -> Self {
        AgentResponseCache {
            cache: Vec::new(),
        }
    }

    pub fn add_response(&mut self, query: &str, response: &str) {
        let query_str = String::from(query);
        let response_str = String::from(response);
        self.cache.push((query_str, response_str));
    }

    pub fn get_response(&self, query: &str) -> Option<&String> {
        for (q, r) in &self.cache {
            if q == query {
                return Some(r);
            }
        }
        None
    }

    pub fn remove_response(&mut self, query: &str) {
        self.cache.retain(|(q, _)| q != query);
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}
