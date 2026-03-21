extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut policy_engine = MdmPolicyEngine::new();

    policy_engine.add_policy(String::from("policy1"), vec![String::from("action1")]);
    policy_engine.add_policy(String::from("policy2"), vec![String::from("action2"), String::from("action3")]);

    if let Some(actions) = policy_engine.get_actions_for_policy("policy1") {
        for action in actions {
            // Simulate executing an action
        }
    }

    policy_engine.remove_policy("policy2");

    if let Some(actions) = policy_engine.get_actions_for_policy("policy2") {
        for action in actions {
            // This should not print anything as the policy was removed
        }
    } else {
    }
}

pub struct MdmPolicyEngine {
    policies: Vec<(String, Vec<String>)>,
}

impl MdmPolicyEngine {
    pub fn new() -> Self {
        MdmPolicyEngine { policies: Vec::new() }
    }

    pub fn add_policy(&mut self, policy_name: String, actions: Vec<String>) {
        self.policies.push((policy_name, actions));
    }

    pub fn remove_policy(&mut self, policy_name: &str) {
        self.policies.retain(|(name, _)| name != policy_name);
    }

    pub fn get_actions_for_policy(&self, policy_name: &str) -> Option<&Vec<String>> {
        self.policies.iter().find_map(|(name, actions)| {
            if name == policy_name {
                Some(actions)
            } else {
                None
            }
        }
    }

    pub fn list_policies(&self) -> Vec<&String> {
        self.policies.iter().map(|(policy_name, _)| policy_name).collect()
    }

    pub fn count_actions(&self, policy_name: &str) -> usize {
        if let Some(actions) = self.get_actions_for_policy(policy_name) {
            actions.len()
        } else {
            0
        }
    }
}
