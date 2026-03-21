extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct EnterpriseAIPolicy {
    policies: Vec<String>,
}

impl EnterpriseAIPolicy {
    pub fn new() -> Self {
        EnterpriseAIPolicy {
            policies: Vec::new(),
        }
    }

    pub fn add_policy(&mut self, policy: String) {
        self.policies.push(policy);
    }

    pub fn remove_policy(&mut self, index: usize) -> Option<String> {
        if index < self.policies.len() {
            Some(self.policies.remove(index))
        } else {
            None
        }
    }

    pub fn get_policy(&self, index: usize) -> Option<&String> {
        self.policies.get(index)
    }

    pub fn list_policies(&self) -> &[String] {
        &self.policies
    }

    pub fn has_policy(&self, policy: &str) -> bool {
        self.policies.iter().any(|p| p == policy)
    }
}
