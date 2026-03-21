extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn security_supply_chain_verify_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn security_supply_chain_verify_exit() {
    // Cleanup logic for the module
}

pub struct SecuritySupplyChainVerifier {
    trusted_sources: Vec<String>,
    verified_components: Vec<String>,
}

impl SecuritySupplyChainVerifier {
    pub fn new(trusted_sources: Vec<String>) -> Self {
        SecuritySupplyChainVerifier {
            trusted_sources,
            verified_components: Vec::new(),
        }
    }

    pub fn add_trusted_source(&mut self, source: String) {
        if !self.trusted_sources.contains(&source) {
            self.trusted_sources.push(source);
        }
    }

    pub fn remove_trusted_source(&mut self, source: &str) -> bool {
        let index = self.trusted_sources.iter().position(|s| s == source);
        if let Some(i) = index {
            self.trusted_sources.remove(i);
            true
        } else {
            false
        }
    }

    pub fn verify_component(&mut self, component: String, source: &str) -> bool {
        if self.trusted_sources.contains(&source.to_string()) {
            self.verified_components.push(component.clone());
            true
        } else {
            false
        }
    }

    pub fn is_verified(&self, component: &str) -> bool {
        self.verified_components.contains(&component.to_string())
    }

    pub fn list_verified_components(&self) -> Vec<String> {
        self.verified_components.clone()
    }
}
