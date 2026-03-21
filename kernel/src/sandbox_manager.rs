extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SandboxPolicy {
    pub can_read_fs: bool,
    pub can_write_fs: bool,
    pub can_network: bool,
    pub can_spawn_agents: bool,
    pub max_memory_bytes: u64,
    pub max_cpu_ms: u64,
    pub allowed_paths: Vec<String>,
    pub blocked_syscalls: Vec<String>
}

pub struct Sandbox {
    pub id: u64,
    pub app_name: String,
    pub policy: SandboxPolicy,
    pub memory_used: u64,
    pub cpu_used_ms: u64,
    pub violations: Vec<String>,
    pub active: bool
}

pub struct SandboxManager {
    pub sandboxes: Vec<Sandbox>,
    pub next_id: u64
}

impl SandboxManager {
    pub fn new() -> Self {
        Self { 
            sandboxes: Vec::new(), 
            next_id: 1 
        }
    }

    pub fn create(&mut self, app_name: &str, policy: SandboxPolicy) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.sandboxes.push(Sandbox { 
            id, 
            app_name: String::from(app_name), 
            policy, 
            memory_used: 0, 
            cpu_used_ms: 0, 
            violations: Vec::new(), 
            active: true 
        };
        id
    }

    pub fn default_policy() -> SandboxPolicy {
        SandboxPolicy { 
            can_read_fs: false, 
            can_write_fs: false, 
            can_network: false, 
            can_spawn_agents: false, 
            max_memory_bytes: 64 * 1024 * 1024, 
            max_cpu_ms: 5000, 
            allowed_paths: Vec::new(), 
            blocked_syscalls: Vec::new() 
        }
    }

    pub fn check_memory(&self, id: u64, requested: u64) -> bool {
        self.sandboxes.iter().find(|s| s.id == id).map(|s| s.memory_used + requested <= s.policy.max_memory_bytes).unwrap_or(false)
    }

    pub fn check_fs_read(&self, id: u64, path: &str) -> bool {
        self.sandboxes.iter().find(|s| s.id == id).map(|s| s.policy.can_read_fs && (s.policy.allowed_paths.is_empty() || s.policy.allowed_paths.iter().any(|p| path.starts_with(p.as_str())))).unwrap_or(false)
    }

    pub fn record_violation(&mut self, id: u64, violation: &str) {
        if let Some(s) = self.sandboxes.iter_mut().find(|s| s.id == id) {
            s.violations.push(String::from(violation));
            if s.violations.len() > 10 {
                s.active = false;
            }
        }
    }

    pub fn kill(&mut self, id: u64) {
        if let Some(s) = self.sandboxes.iter_mut().find(|s| s.id == id) {
            s.active = false;
        }
    }
)}
