extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Capability {
    pub name: String,
    pub granted: bool,
    pub trust_score: f32,
    pub expires_at: Option<u64>,
}

pub struct FilePermission {
    pub file_id: u64,
    pub owner_agent: u64,
    pub capabilities: Vec<Capability>,
    pub shared_with: Vec<u64>,
    pub encrypted: bool,
    pub audit_log: Vec<AccessLog>,
}

pub struct AccessLog {
    pub agent_id: u64,
    pub action: String,
    pub timestamp: u64,
    pub allowed: bool,
}

pub struct PermissionEngine {
    pub permissions: Vec<FilePermission>,
    pub default_trust: f32,
}

impl PermissionEngine {
    pub fn new() -> Self {
        Self {
            permissions: Vec::new(),
            default_trust: 0.5,
        }
    }

    pub fn register_file(&mut self, file_id: u64, owner: u64) {
        self.permissions.push(FilePermission {
            file_id,
            owner_agent: owner,
            capabilities: Vec::new(),
            shared_with: Vec::new(),
            encrypted: false,
            audit_log: Vec::new(),
        };
    }

    pub fn grant(&mut self, file_id: u64, cap_name: &str, trust: f32) {
        if let Some(perm) = self.permissions.iter_mut().find(|p| p.file_id == file_id) {
            perm.capabilities.push(Capability {
                name: String::from(cap_name),
                granted: true,
                trust_score: trust,
                expires_at: None,
            });
        }
    }

    pub fn check_access(&mut self, file_id: u64, agent_id: u64, action: &str) -> bool {
        if let Some(perm) = self.permissions.iter_mut().find(|p| p.file_id == file_id) {
            let allowed = perm.owner_agent == agent_id
                || perm.shared_with.contains(&agent_id)
                || perm.capabilities.iter().any(|c| c.name == action && c.granted && c.trust_score >= self.default_trust);
            perm.audit_log.push(AccessLog {
                agent_id,
                action: String::from(action),
                timestamp: 0,
                allowed,
            });
            allowed
        } else {
            false
        }
    }

    pub fn share_file(&mut self, file_id: u64, with_agent: u64) {
        if let Some(perm) = self.permissions.iter_mut().find(|p| p.file_id == file_id) {
            if !perm.shared_with.contains(&with_agent) {
                perm.shared_with.push(with_agent);
            }
        }
    }

    pub fn revoke_share(&mut self, file_id: u64, agent_id: u64) {
        if let Some(perm) = self.permissions.iter_mut().find(|p| p.file_id == file_id) {
            perm.shared_with.retain(|a| *a != agent_id);
        }
    }
)}
