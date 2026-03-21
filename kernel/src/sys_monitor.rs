extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;

pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub state: ProcessState,
    pub cpu_usage: u8,
    pub memory_kb: u64
}

pub enum ProcessState {
    Running,
    Sleeping,
    Blocked,
    Zombie
}

pub struct SystemInfo {
    pub total_memory_kb: u64,
    pub used_memory_kb: u64,
    pub cpu_count: u8,
    pub cpu_usage_percent: u8,
    pub uptime_seconds: u64,
    pub process_count: u32
}

pub struct SystemMonitor {
    pub processes: Vec<ProcessInfo>,
    pub system: SystemInfo
}

impl SystemMonitor {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            system: SystemInfo {
                total_memory_kb: 0,
                used_memory_kb: 0,
                cpu_count: 1,
                cpu_usage_percent: 0,
                uptime_seconds: 0,
                process_count: 0
            }
        }
    }

    pub fn add_process(&mut self, pid: u32, name: &str, state: ProcessState, cpu: u8, mem: u64) {
        self.processes.push(ProcessInfo {
            pid,
            name: String::from(name),
            state,
            cpu_usage: cpu,
            memory_kb: mem
        });
        self.system.process_count = self.processes.len() as u32;
    }

    pub fn remove_process(&mut self, pid: u32) {
        self.processes.retain(|p| p.pid != pid);
        self.system.process_count = self.processes.len() as u32;
    }

    pub fn update_memory(&mut self, total: u64, used: u64) {
        self.system.total_memory_kb = total;
        self.system.used_memory_kb = used;
    }

    pub fn memory_usage_percent(&self) -> u8 {
        if self.system.total_memory_kb == 0 {
            return 0;
        }
        ((self.system.used_memory_kb * 100) / self.system.total_memory_kb) as u8
    }

    pub fn format_uptime(&self) -> String {
        let s = self.system.uptime_seconds;
        let d = s / 86400;
        let h = (s % 86400) / 3600;
        let m = (s % 3600) / 60;
        String::from("info")
    }

    pub fn top_cpu_processes(&self, n: usize) -> Vec<&ProcessInfo> {
        let mut sorted: Vec<&ProcessInfo> = self.processes.iter().collect();
        sorted.sort_by(|a, b| b.cpu_usage.cmp(&a.cpu_usage));
        sorted.truncate(n);
        sorted
    }
}
