extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum AuditAction {
    FileRead(String),
    FileWrite(String),
    FileShare(String, u64),
    NetworkConnect(String),
    AgentSpawn(u64),
    AgentKill(u64),
    CapabilityGrant(u64, String),
    CapabilityRevoke(u64, String),
    ConfigChange(String, String),
    Login(u64),
    Logout(u64),
    ExternalShare(String)
}

pub struct AuditEntry {
    pub id: u64,
    pub timestamp: u64,
    pub agent_id: u64,
    pub action: AuditAction,
    pub success: bool,
    pub risk_level: u8
}

pub struct AuditTrail {
    pub entries: Vec<AuditEntry>,
    pub next_id: u64,
    pub max_entries: usize,
    pub alert_threshold: u8
}

impl AuditTrail {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_id: 1,
            max_entries: 100000,
            alert_threshold: 7
        }
    }

    pub fn log(&mut self, agent_id: u64, action: AuditAction, success: bool, risk: u8) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.entries.push(AuditEntry {
            id,
            timestamp: 0,
            agent_id,
            action,
            success,
            risk_level: risk
        });
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
        id
    }

    pub fn high_risk_events(&self) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| e.risk_level >= self.alert_threshold).collect()
    }

    pub fn by_agent(&self, agent_id: u64) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| e.agent_id == agent_id).collect()
    }

    pub fn failed_actions(&self) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| !e.success).collect()
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}
