extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AIProviderVersionMgr {
    providers: Vec<String>,
}

impl AIProviderVersionMgr {
    pub fn new() -> Self {
        AIProviderVersionMgr {
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

    pub fn get_providers(&self) -> Vec<String> {
        self.providers.clone()
    }

    pub fn has_provider(&self, provider_name: &str) -> bool {
        self.providers.contains(&String::from(provider_name))
    }

    pub fn count_providers(&self) -> usize {
        self.providers.len()
    }
}
