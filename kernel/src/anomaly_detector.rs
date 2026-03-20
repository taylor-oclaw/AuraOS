extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum AnomalyType {
    HighCpu,
    HighMemory,
    ExcessiveApiCalls,
    UnusualPattern,
    SecurityViolation,
    CostSpike,
    SlowResponse,
}

pub struct Anomaly {
    pub id: u64,
    pub agent_id: u64,
    pub anomaly_type: AnomalyType,
    pub severity: u8,
    pub description: String,
    pub detected_at: u64,
    pub resolved: bool,
}

pub struct AnomalyDetector {
    pub anomalies: Vec<Anomaly>,
    pub next_id: u64,
    pub thresholds: AnomalyThresholds,
    pub enabled: bool,
}

pub struct AnomalyThresholds {
    pub cpu_pct: f32,
    pub memory_bytes: u64,
    pub api_calls_per_min: u32,
    pub response_ms: u64,
    pub cost_per_hour: u64,
}

impl AnomalyDetector {
    pub fn new() -> Self {
        Self {
            anomalies: Vec::new(),
            next_id: 1,
            thresholds: AnomalyThresholds {
                cpu_pct: 90.0,
                memory_bytes: 1073741824,
                api_calls_per_min: 100,
                response_ms: 5000,
                cost_per_hour: 1000,
            },
            enabled: true,
        }
    }

    pub fn detect(&mut self, agent_id: u64, atype: AnomalyType, severity: u8, desc: &str) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.anomalies.push(Anomaly {
            id,
            agent_id,
            anomaly_type: atype,
            severity,
            description: String::from(desc),
            detected_at: 0,
            resolved: false,
        });
        id
    }

    pub fn resolve(&mut self, id: u64) {
        if let Some(a) = self.anomalies.iter_mut().find(|a| a.id == id) {
            a.resolved = true;
        }
    }

    pub fn active_anomalies(&self) -> Vec<&Anomaly> {
        self.anomalies.iter().filter(|a| !a.resolved).collect()
    }

    pub fn critical_anomalies(&self) -> Vec<&Anomaly> {
        self.anomalies.iter().filter(|a| !a.resolved && a.severity >= 8).collect()
    }

    pub fn anomalies_for_agent(&self, agent_id: u64) -> Vec<&Anomaly> {
        self.anomalies.iter().filter(|a| a.agent_id == agent_id).collect()
    }

    pub fn total(&self) -> usize {
        self.anomalies.len()
    }
}
