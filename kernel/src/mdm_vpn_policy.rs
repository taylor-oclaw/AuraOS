extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mdm_vpn_policy_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mdm_vpn_policy_exit() {
    // Cleanup logic for the module
}

pub struct VPNPolicy {
    policies: Vec<String>,
}

impl VPNPolicy {
    pub fn new() -> Self {
        VPNPolicy {
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

    pub fn clear_policies(&mut self) {
        self.policies.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vpn_policy() {
        let mut policy = VPNPolicy::new();

        assert_eq!(policy.list_policies().len(), 0);

        policy.add_policy(String::from("AllowVPN"));
        policy.add_policy(String::from("BlockVPN"));

        assert_eq!(policy.list_policies().len(), 2);
        assert_eq!(policy.get_policy(0), Some(&String::from("AllowVPN")));
        assert_eq!(policy.get_policy(1), Some(&String::from("BlockVPN")));

        let removed = policy.remove_policy(0);
        assert_eq!(removed, Some(String::from("AllowVPN")));
        assert_eq!(policy.list_policies().len(), 1);

        policy.clear_policies();
        assert_eq!(policy.list_policies().len(), 0);
    }
}
