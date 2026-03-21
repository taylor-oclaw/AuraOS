extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ResourceSnapshot {
    pub timestamp: u64,
    pub cpu_usage_pct: f32,
    pub memory_used_bytes: u64,
    pub memory_total_bytes: u64,
    pub disk_used_bytes: u64,
    pub agent_count: u32,
    pub active_tasks: u32
}

pub struct AgentResources {
    pub agent_id: u64,
    pub cpu_pct: f32,
    pub memory_bytes: u64,
    pub io_reads: u64,
    pub io_writes: u64,
    pub network_bytes_sent: u64,
    pub network_bytes_recv: u64
}

pub struct ResourceMonitor {
    pub snapshots: Vec<ResourceSnapshot>,
    pub agent_usage: Vec<AgentResources>,
    pub max_snapshots: usize,
    pub alert_threshold_cpu: f32,
    pub alert_threshold_memory_pct: f32
}

impl ResourceMonitor {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            agent_usage: Vec::new(),
            max_snapshots: 1000,
            alert_threshold_cpu: 90.0,
            alert_threshold_memory_pct: 85.0
        }
    }

    pub fn record_snapshot(&mut self, cpu: f32, mem_used: u64, mem_total: u64, disk: u64, agents: u32, tasks: u32) {
        self.snapshots.push(ResourceSnapshot {
            timestamp: 0,
            cpu_usage_pct: cpu,
            memory_used_bytes: mem_used,
            memory_total_bytes: mem_total,
            disk_used_bytes: disk,
            agent_count: agents,
            active_tasks: tasks
        };
        if self.snapshots.len() > self.max_snapshots {
            self.snapshots.remove(0);
        }
    }

    pub fn update_agent(&mut self, agent_id: u64, cpu: f32, mem: u64) {
        if let Some(a) = self.agent_usage.iter_mut().find(|a| a.agent_id == agent_id) {
            a.cpu_pct = cpu;
            a.memory_bytes = mem;
        } else {
            self.agent_usage.push(AgentResources {
                agent_id,
                cpu_pct: cpu,
                memory_bytes: mem,
                io_reads: 0,
                io_writes: 0,
                network_bytes_sent: 0,
                network_bytes_recv: 0
            };
        }
    }

    pub fn is_critical(&self) -> bool {
        self.snapshots.last().map(|s| s.cpu_usage_pct > self.alert_threshold_cpu).unwrap_or(false)
    }

    pub fn top_agents_by_cpu(&self, n: usize) -> Vec<&AgentResources> {
        let mut sorted: Vec<&AgentResources> = self.agent_usage.iter().collect();
        sorted.sort_by(|a, b| b.cpu_pct.partial_cmp(&a.cpu_pct).unwrap_or(core::cmp::Ordering::Equal));
        sorted.truncate(n);
        sorted
    }

    pub fn total_memory_used(&self) -> u64 {
        self.agent_usage.iter().map(|a| a.memory_bytes).sum()
    }
))}
