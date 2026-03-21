extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct EnterpriseModelAllowlist {
    allowlist: Vec<String>,
}

impl EnterpriseModelAllowlist {
    pub fn new() -> Self {
        EnterpriseModelAllowlist {
            allowlist: Vec::new(),
        }
    }

    pub fn add_model(&mut self, model_name: &str) {
        if !self.allowlist.contains(&model_name.to_string()) {
            self.allowlist.push(model_name.to_string());
        }
    }

    pub fn remove_model(&mut self, model_name: &str) {
        self.allowlist.retain(|m| m != model_name);
    }

    pub fn is_allowed(&self, model_name: &str) -> bool {
        self.allowlist.contains(&model_name.to_string())
    }

    pub fn list_models(&self) -> Vec<String> {
        self.allowlist.clone()
    }

    pub fn clear_allowlist(&mut self) {
        self.allowlist.clear();
    }
}
