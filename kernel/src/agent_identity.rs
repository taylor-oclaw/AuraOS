extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentCert {
    pub agent_id: u64,
    pub public_key: [u8; 32],
    pub issuer: String,
    pub valid_from: u64,
    pub valid_until: u64,
    pub capabilities: Vec<String>,
    pub trust_level: f32,
    pub revoked: bool
}

pub struct IdentityManager {
    pub certificates: Vec<AgentCert>,
    pub root_key: [u8; 32],
    pub next_id: u64
}

impl IdentityManager {
    pub fn new() -> Self {
        Self {
            certificates: Vec::new(),
            root_key: [0; 32],
            next_id: 1
        }
    }

    pub fn issue_cert(&mut self, capabilities: Vec<String>, trust: f32, valid_secs: u64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.certificates.push(AgentCert {
            agent_id: id,
            public_key: [0; 32],
            issuer: String::from("AuraOS-Root"),
            valid_from: 0,
            valid_until: valid_secs,
            capabilities,
            trust_level: trust,
            revoked: false
        };
        id
    }

    pub fn verify(&self, agent_id: u64) -> bool {
        self.certificates.iter().any(|c| c.agent_id == agent_id && !c.revoked && c.trust_level > 0.0)
    }

    pub fn revoke(&mut self, agent_id: u64) {
        if let Some(c) = self.certificates.iter_mut().find(|c| c.agent_id == agent_id) {
            c.revoked = true;
        }
    }

    pub fn get_capabilities(&self, agent_id: u64) -> Vec<&str> {
        self.certificates.iter().find(|c| c.agent_id == agent_id && !c.revoked).map(|c| c.capabilities.iter().map(|s| s.as_str()).collect()).unwrap_or_default()
    }

    pub fn trusted_agents(&self) -> Vec<u64> {
        self.certificates.iter().filter(|c| !c.revoked && c.trust_level > 0.5).map(|c| c.agent_id).collect()
    }
)}
