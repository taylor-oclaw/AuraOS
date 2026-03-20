extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AgentPolicyEngine {
    policies: Vec<String>,
    active_policy_index: usize,
}

impl AgentPolicyEngine {
    pub fn new() -> Self {
        AgentPolicyEngine {
            policies: Vec::new(),
            active_policy_index: 0,
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

    pub fn get_active_policy(&self) -> Option<&String> {
        if self.active_policy_index < self.policies.len() {
            Some(&self.policies[self.active_policy_index])
        } else {
            None
        }
    }

    pub fn set_active_policy(&mut self, index: usize) -> bool {
        if index < self.policies.len() {
            self.active_policy_index = index;
            true
        } else {
            false
        }
    }

    pub fn list_policies(&self) -> &[String] {
        &self.policies
    }
}
