extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct McpCapabilityNegotiate {
    capabilities: Vec<String>,
}

impl McpCapabilityNegotiate {
    pub fn new() -> Self {
        McpCapabilityNegotiate {
            capabilities: Vec::new(),
        }
    }

    pub fn add_capability(&mut self, capability: String) {
        self.capabilities.push(capability);
    }

    pub fn remove_capability(&mut self, capability: &str) {
        if let Some(index) = self.capabilities.iter().position(|c| c == capability) {
            self.capabilities.remove(index);
        }
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
