extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;
use alloc::format;

pub struct SystemInfo {
    os_name: String,
    os_version: String,
    arch: String,
    cpu_cores: u32,
    memory_total_kb: u64,
    memory_used_kb: u64,
    uptime_seconds: u64,
    hostname: String,
}

impl SystemInfo {
    pub fn new() -> Self {
        SystemInfo {
            os_name: String::from("AuraOS"),
            os_version: String::from("1.0"),
            arch: String::from("x86_64"),
            cpu_cores: 4,
            memory_total_kb: 16384, // 16 GB
            memory_used_kb: 0,
            uptime_seconds: 0,
            hostname: String::from("aura-host"),
        }
    }

    pub fn update_memory(&mut self, used_kb: u64) {
        self.memory_used_kb = used_kb;
    }

    pub fn tick_uptime(&mut self) {
        self.uptime_seconds += 1;
    }

    pub fn memory_free_kb(&self) -> u64 {
        self.memory_total_kb - self.memory_used_kb
    }

    pub fn memory_usage_percent(&self) -> u8 {
        if self.memory_total_kb == 0 {
            return 0;
        }
        ((self.memory_used_kb as f64 / self.memory_total_kb as f64) * 100.0) as u8
    }

    pub fn summary(&self) -> Vec<String> {
        vec![
            String::from("info"),
            String::from("info"),
            String::from("info"),
            String::from("info"),
            String::from("info"),
            String::from("info"),
            String::from("info")),
            String::from("info")),
            String::from("info"),
            String::from("info"),
        ]
    }
}
