extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MdmNetworkPolicy {
    policy_id: u32,
    network_type: String,
    ip_address: String,
}

impl MdmNetworkPolicy {
    pub fn new(policy_id: u32, network_type: &str, ip_address: &str) -> Self {
        MdmNetworkPolicy {
            policy_id,
            network_type: String::from(network_type),
            ip_address: String::from(ip_address),
        }
    }

    pub fn get_policy_id(&self) -> u32 {
        self.policy_id
    }

    pub fn set_network_type(&mut self, new_type: &str) {
        self.network_type = String::from(new_type);
    }

    pub fn get_network_type(&self) -> &str {
        &self.network_type
    }

    pub fn update_ip_address(&mut self, new_ip: &str) {
        self.ip_address = String::from(new_ip);
    }

    pub fn get_ip_address(&self) -> &str {
        &self.ip_address
    }
}

pub struct MdmNetworkPolicyList {
    policies: Vec<MdmNetworkPolicy>,
}

impl MdmNetworkPolicyList {
    pub fn new() -> Self {
        MdmNetworkPolicyList { policies: Vec::new() }
    }

    pub fn add_policy(&mut self, policy: MdmNetworkPolicy) {
        self.policies.push(policy);
    }

    pub fn get_policies(&self) -> &Vec<MdmNetworkPolicy> {
        &self.policies
    }

    pub fn remove_policy(&mut self, policy_id: u32) -> bool {
        for (i, policy) in self.policies.iter().enumerate() {
            if policy.get_policy_id() == policy_id {
                self.policies.remove(i);
                return true;
            }
        }
        false
    }

    pub fn get_policy(&self, policy_id: u32) -> Option<&MdmNetworkPolicy> {
        for policy in &self.policies {
            if policy.get_policy_id() == policy_id {
                return Some(policy);
            }
        }
        None
    }
}