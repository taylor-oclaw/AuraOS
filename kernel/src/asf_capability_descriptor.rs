extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AsfCapabilityDescriptor {
    capabilities: Vec<String>,
}

impl AsfCapabilityDescriptor {
    pub fn new() -> Self {
        AsfCapabilityDescriptor {
            capabilities: Vec::new(),
        }
    }

    pub fn add_capability(&mut self, capability: &str) {
        self.capabilities.push(String::from(capability));
    }

    pub fn has_capability(&self, capability: &str) -> bool {
        self.capabilities.contains(&String::from(capability))
    }

    pub fn remove_capability(&mut self, capability: &str) {
        if let Some(index) = self.capabilities.iter().position(|c| c == capability) {
            self.capabilities.remove(index);
        }
    }

    pub fn list_capabilities(&self) -> Vec<String> {
        self.capabilities.clone()
    }

    pub fn count_capabilities(&self) -> usize {
        self.capabilities.len()
    }
}
