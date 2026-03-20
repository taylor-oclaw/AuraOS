extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CapabilitySystem {
    capabilities: Vec<String>,
}

impl CapabilitySystem {
    pub fn new() -> Self {
        CapabilitySystem {
            capabilities: Vec::new(),
        }
    }

    pub fn add_capability(&mut self, capability: &str) {
        if !self.capabilities.contains(&String::from(capability)) {
            self.capabilities.push(String::from(capability));
        }
    }

    pub fn remove_capability(&mut self, capability: &str) {
        self.capabilities.retain(|c| c != capability);
    }

    pub fn has_capability(&self, capability: &str) -> bool {
        self.capabilities.contains(&String::from(capability))
    }

    pub fn list_capabilities(&self) -> Vec<String> {
        self.capabilities.clone()
    }

    pub fn clear_capabilities(&mut self) {
        self.capabilities.clear();
    }
}
