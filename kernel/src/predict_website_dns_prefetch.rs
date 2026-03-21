extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DNSPrefetcher {
    domain_list: Vec<String>,
}

impl DNSPrefetcher {
    pub fn new() -> Self {
        DNSPrefetcher {
            domain_list: Vec::new(),
        }
    }

    pub fn add_domain(&mut self, domain: String) {
        if !self.domain_list.contains(&domain) {
            self.domain_list.push(domain);
        }
    }

    pub fn remove_domain(&mut self, domain: &str) -> bool {
        let index = self.domain_list.iter().position(|d| d == domain);
        match index {
            Some(i) => {
                self.domain_list.remove(i);
                true
            }
            None => false,
        }
    }

    pub fn get_domains(&self) -> Vec<String> {
        self.domain_list.clone()
    }

    pub fn contains_domain(&self, domain: &str) -> bool {
        self.domain_list.contains(&String::from(domain))
    }

    pub fn clear_domains(&mut self) {
        self.domain_list.clear();
    }
}
