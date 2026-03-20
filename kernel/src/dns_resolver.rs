extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DnsResolver {
    // Placeholder for DNS resolver state
    cache: Vec<(String, String)>,
}

impl DnsResolver {
    pub fn new() -> Self {
        DnsResolver {
            cache: Vec::new(),
        }
    }

    pub fn resolve(&mut self, domain: &str) -> Option<&str> {
        for (d, ip) in &self.cache {
            if d == domain {
                return Some(ip);
            }
        }
        None
    }

    pub fn add_record(&mut self, domain: String, ip: String) {
        self.cache.push((domain, ip));
    }

    pub fn remove_record(&mut self, domain: &str) {
        self.cache.retain(|(d, _)| d != domain);
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    pub fn list_records(&self) -> Vec<(String, String)> {
        self.cache.clone()
    }
}
