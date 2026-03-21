extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
}

pub struct PrivacyPolicy {
    pub agent_id: u64,
    pub allowed_data: Vec<DataClassification>,
    pub can_share_externally: bool,
    pub data_retention_hours: u64,
    pub anonymize_logs: bool,
}

pub struct PrivacyEngine {
    pub policies: Vec<PrivacyPolicy>,
    pub violations: Vec<PrivacyViolation>,
    pub next_id: u64,
}

pub struct PrivacyViolation {
    pub id: u64,
    pub agent_id: u64,
    pub description: String,
    pub severity: u8,
    pub timestamp: u64,
}

impl PrivacyEngine {
    pub fn new() -> Self {
        Self {
            policies: Vec::new(),
            violations: Vec::new(),
            next_id: 1,
        }
    }

    pub fn set_policy(
        &mut self,
        agent_id: u64,
        allowed: Vec<DataClassification>,
        share: bool,
        retention: u64,
    ) {
        self.policies.push(PrivacyPolicy {
            agent_id,
            allowed_data: allowed,
            can_share_externally: share,
            data_retention_hours: retention,
            anonymize_logs: true,
        });
    }

    pub fn check_access(&mut self, agent_id: u64, classification: &DataClassification) -> bool {
        if let Some(policy) = self.policies.iter().find(|p| p.agent_id == agent_id) {
            let allowed = policy.allowed_data.iter().any(|c| core::mem::discriminant(c) == core::mem::discriminant(classification));
            if !allowed {
                let id = self.next_id;
                self.next_id += 1;
                self.violations.push(PrivacyViolation {
                    id,
                    agent_id,
                    description: String::from("Unauthorized data access attempt"),
                    severity: 8,
                    timestamp: 0,
                });
            }
            allowed
        } else {
            false
        }
    }

    pub fn violation_count(&self) -> usize {
        self.violations.len()
    }

    pub fn agent_violations(&self, agent_id: u64) -> Vec<&PrivacyViolation> {
        self.violations.iter().filter(|v| v.agent_id == agent_id).collect()
    }
}
