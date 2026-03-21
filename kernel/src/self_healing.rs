extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum HealthStatus {
    Healthy,
    Degraded,
    Failed,
    Recovering,
    Unknown,
}

pub struct AgentHealth {
    pub agent_id: u64,
    pub status: HealthStatus,
    pub last_heartbeat: u64,
    pub restart_count: u32,
    pub max_restarts: u32,
    pub error_log: Vec<String>,
    pub uptime_secs: u64,
}

pub struct SelfHealing {
    pub agents: Vec<AgentHealth>,
    pub check_interval_secs: u64,
    pub heartbeat_timeout_secs: u64,
    pub auto_restart: bool,
    pub total_recoveries: u64,
}

impl SelfHealing {
    pub fn new() -> Self {
        Self {
            agents: Vec::new(),
            check_interval_secs: 30,
            heartbeat_timeout_secs: 60,
            auto_restart: true,
            total_recoveries: 0,
        }
    }

    pub fn register(&mut self, agent_id: u64) {
        self.agents.push(AgentHealth {
            agent_id,
            status: HealthStatus::Healthy,
            last_heartbeat: 0,
            restart_count: 0,
            max_restarts: 5,
            error_log: Vec::new(),
            uptime_secs: 0,
        };
    }

    pub fn heartbeat(&mut self, agent_id: u64) {
        if let Some(a) = self.agents.iter_mut().find(|a| a.agent_id == agent_id) {
            a.last_heartbeat = 0;
            a.status = HealthStatus::Healthy;
        }
    }

    pub fn report_failure(&mut self, agent_id: u64, error: &str) {
        if let Some(a) = self.agents.iter_mut().find(|a| a.agent_id == agent_id) {
            a.status = HealthStatus::Failed;
            a.error_log.push(String::from(error));
        }
    }

    pub fn attempt_recovery(&mut self, agent_id: u64) -> bool {
        if let Some(a) = self.agents.iter_mut().find(|a| a.agent_id == agent_id) {
            if a.restart_count < a.max_restarts {
                a.restart_count += 1;
                a.status = HealthStatus::Recovering;
                self.total_recoveries += 1;
                return true;
            }
        }
        false
    }

    pub fn failed_agents(&self) -> Vec<u64> {
        self.agents.iter().filter(|a| matches!(a.status, HealthStatus::Failed)).map(|a| a.agent_id).collect()
    }

    pub fn healthy_count(&self) -> usize {
        self.agents.iter().filter(|a| matches!(a.status, HealthStatus::Healthy)).count()
    }
)}
