extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AppDomSelector {
    domains: Vec<String>,
}

impl AppDomSelector {
    pub fn new() -> Self {
        AppDomSelector {
            domains: Vec::new(),
        }
    }

    pub fn add_domain(&mut self, domain: String) {
        if !self.domains.contains(&domain) {
            self.domains.push(domain);
        }
    }

    pub fn remove_domain(&mut self, domain: &str) {
        self.domains.retain(|d| d != domain);
    }

    pub fn get_domains(&self) -> Vec<String> {
        self.domains.clone()
    }

    pub fn has_domain(&self, domain: &str) -> bool {
        self.domains.contains(&String::from(domain))
    }

    pub fn count_domains(&self) -> usize {
        self.domains.len()
    }
}
