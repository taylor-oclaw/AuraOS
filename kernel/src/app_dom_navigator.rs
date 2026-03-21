extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let navigator = AppDomNavigator::new();
    navigator.initialize();
    loop {}
}

pub struct AppDomNavigator {
    current_domain: String,
    domains: Vec<String>,
}

impl AppDomNavigator {
    pub fn new() -> Self {
        AppDomNavigator {
            current_domain: String::from("default"),
            domains: vec![String::from("default")],
        }
    }

    pub fn add_domain(&mut self, domain_name: &str) {
        if !self.domains.contains(&String::from(domain_name)) {
            self.domains.push(String::from(domain_name));
        }
    }

    pub fn remove_domain(&mut self, domain_name: &str) {
        self.domains.retain(|d| d != domain_name);
        if self.current_domain == domain_name {
            self.current_domain = String::from("default");
        }
    }

    pub fn switch_to_domain(&mut self, domain_name: &str) -> bool {
        if self.domains.contains(&String::from(domain_name)) {
            self.current_domain = String::from(domain_name);
            true
        } else {
            false
        }
    }

    pub fn list_domains(&self) -> Vec<String> {
        self.domains.clone()
    }

    pub fn initialize(&mut self) {
        // Initialization logic for the navigator
    }
}
