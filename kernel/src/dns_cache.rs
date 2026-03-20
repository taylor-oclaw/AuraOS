extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DnsCache {
    entries: Vec<(String, String)>,
}

impl DnsCache {
    pub fn new() -> Self {
        DnsCache {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, domain: &str, ip: &str) {
        let domain = String::from(domain);
        let ip = String::from(ip);
        self.entries.push((domain, ip));
    }

    pub fn get_ip(&self, domain: &str) -> Option<&String> {
        for (d, ip) in &self.entries {
            if d == domain {
                return Some(ip);
            }
        }
        None
    }

    pub fn remove_entry(&mut self, domain: &str) {
        self.entries.retain(|(d, _)| d != domain);
    }

    pub fn clear_cache(&mut self) {
        self.entries.clear();
    }

    pub fn list_entries(&self) -> Vec<(String, String)> {
        self.entries.iter().map(|(d, ip)| (d.clone(), ip.clone())).collect()
    }
}
