extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn init_module() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() -> i32 {
    0
}

pub struct AgentTimeoutPolicy {
    policy_name: String,
    timeout_duration: u64, // in milliseconds
    max_retries: u32,
    retry_interval: u64, // in milliseconds
    active_agents: Vec<String>,
}

impl AgentTimeoutPolicy {
    pub fn new(name: &str, timeout: u64, retries: u32, interval: u64) -> Self {
        AgentTimeoutPolicy {
            policy_name: String::from(name),
            timeout_duration: timeout,
            max_retries: retries,
            retry_interval: interval,
            active_agents: Vec::new(),
        }
    }

    pub fn add_agent(&mut self, agent_id: &str) {
        if !self.active_agents.contains(&String::from(agent_id)) {
            self.active_agents.push(String::from(agent_id));
        }
    }

    pub fn remove_agent(&mut self, agent_id: &str) {
        self.active_agents.retain(|agent| agent != agent_id);
    }

    pub fn get_timeout_duration(&self) -> u64 {
        self.timeout_duration
    }

    pub fn set_timeout_duration(&mut self, timeout: u64) {
        self.timeout_duration = timeout;
    }

    pub fn check_agent_status(&self, agent_id: &str) -> bool {
        self.active_agents.contains(&String::from(agent_id))
    }
}
