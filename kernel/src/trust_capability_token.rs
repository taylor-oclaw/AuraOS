extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct TrustCapabilityToken {
    token_id: String,
    capabilities: Vec<String>,
    owner: String,
    expiration_time: u64,
}

impl TrustCapabilityToken {
    pub fn new(token_id: String, capabilities: Vec<String>, owner: String, expiration_time: u64) -> Self {
        TrustCapabilityToken {
            token_id,
            capabilities,
            owner,
            expiration_time,
        }
    }

    pub fn get_token_id(&self) -> &str {
        &self.token_id
    }

    pub fn get_capabilities(&self) -> &Vec<String> {
        &self.capabilities
    }

    pub fn get_owner(&self) -> &str {
        &self.owner
    }

    pub fn is_expired(&self, current_time: u64) -> bool {
        current_time > self.expiration_time
    }

    pub fn add_capability(&mut self, capability: String) {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
        }
    }

    pub fn remove_capability(&mut self, capability: &str) {
        self.capabilities.retain(|c| c != capability);
    }
}
