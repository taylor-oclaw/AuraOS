extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AIProviderDiscovery {
    providers: Vec<String>,
}

impl AIProviderDiscovery {
    pub fn new() -> Self {
        AIProviderDiscovery {
            providers: Vec::new(),
        }
    }

    pub fn add_provider(&mut self, provider_name: &str) {
        self.providers.push(String::from(provider_name));
    }

    pub fn remove_provider(&mut self, provider_name: &str) {
        if let Some(index) = self.providers.iter().position(|p| p == provider_name) {
            self.providers.remove(index);
        }
    }

    pub fn list_providers(&self) -> Vec<String> {
        self.providers.clone()
    }

    pub fn find_provider(&self, provider_name: &str) -> Option<&String> {
        self.providers.iter().find(|&&p| p == provider_name)
    }

    pub fn count_providers(&self) -> usize {
        self.providers.len()
    }
}