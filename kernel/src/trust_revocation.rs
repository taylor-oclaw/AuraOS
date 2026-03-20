extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct TrustRevocation {
    revoked_list: Vec<String>,
}

impl TrustRevocation {
    pub fn new() -> Self {
        TrustRevocation {
            revoked_list: Vec::new(),
        }
    }

    pub fn add_revoked(&mut self, identifier: &str) {
        if !self.revoked_list.contains(&identifier.to_string()) {
            self.revoked_list.push(identifier.to_string());
        }
    }

    pub fn remove_revoked(&mut self, identifier: &str) {
        self.revoked_list.retain(|id| id != identifier);
    }

    pub fn is_revoked(&self, identifier: &str) -> bool {
        self.revoked_list.contains(&identifier.to_string())
    }

    pub fn list_all_revoked(&self) -> Vec<String> {
        self.revoked_list.clone()
    }

    pub fn clear_all_revoked(&mut self) {
        self.revoked_list.clear();
    }
}
