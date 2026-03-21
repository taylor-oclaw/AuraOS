extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentVersion {
    pub version: u32,
    pub config_snapshot: String,
    pub capabilities: Vec<String>,
    pub created_at: u64,
    pub active: bool,
    pub performance_score: f32
}

pub struct AgentVersioning {
    pub agent_id: u64,
    pub versions: Vec<AgentVersion>,
    pub current_version: u32,
    pub auto_rollback: bool
}

impl AgentVersioning {
    pub fn new(agent_id: u64) -> Self {
        Self {
            agent_id,
            versions: Vec::new(),
            current_version: 0,
            auto_rollback: true
        }
    }

    pub fn checkpoint(&mut self, config: &str, capabilities: Vec<String>, score: f32) -> u32 {
        let version = self.versions.len() as u32 + 1;
        for v in &mut self.versions {
            v.active = false;
        }
        self.versions.push(AgentVersion {
            version,
            config_snapshot: String::from(config),
            capabilities,
            created_at: 0,
            active: true,
            performance_score: score
        });
        self.current_version = version;
        version
    }

    pub fn rollback(&mut self, version: u32) -> bool {
        if let Some(v) = self.versions.iter().find(|v| v.version == version) {
            if !v.active {
                for ver in &mut self.versions {
                    ver.active = ver.version == version;
                }
                self.current_version = version;
                return true;
            }
        }
        false
    }

    pub fn rollback_to_best(&mut self) -> Option<u32> {
        let best = self.versions.iter().max_by(|a, b| a.performance_score.partial_cmp(&b.performance_score).unwrap_or(core::cmp::Ordering::Equal));
        if let Some(b) = best {
            let ver = b.version;
            self.rollback(ver);
            Some(ver)
        } else {
            None
        }
    }

    pub fn current(&self) -> Option<&AgentVersion> {
        self.versions.iter().find(|v| v.active)
    }

    pub fn version_count(&self) -> usize {
        self.versions.len()
    }
}
