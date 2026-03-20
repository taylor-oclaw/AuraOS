extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraSystemInfo {
    cpu_cores: usize,
    memory_size_mb: usize,
    disk_space_gb: usize,
    os_version: String,
    uptime_seconds: usize,
}

impl AuraSystemInfo {
    pub fn new(cpu_cores: usize, memory_size_mb: usize, disk_space_gb: usize, os_version: &str, uptime_seconds: usize) -> Self {
        AuraSystemInfo {
            cpu_cores,
            memory_size_mb,
            disk_space_gb,
            os_version: String::from(os_version),
            uptime_seconds,
        }
    }

    pub fn get_cpu_cores(&self) -> usize {
        self.cpu_cores
    }

    pub fn get_memory_size_mb(&self) -> usize {
        self.memory_size_mb
    }

    pub fn get_disk_space_gb(&self) -> usize {
        self.disk_space_gb
    }

    pub fn get_os_version(&self) -> &str {
        &self.os_version
    }

    pub fn get_uptime_seconds(&self) -> usize {
        self.uptime_seconds
    }

    pub fn is_low_memory(&self, threshold_mb: usize) -> bool {
        self.memory_size_mb < threshold_mb
    }

    pub fn disk_space_below_threshold(&self, threshold_gb: usize) -> bool {
        self.disk_space_gb < threshold_gb
    }

    pub fn uptime_in_minutes(&self) -> usize {
        self.uptime_seconds / 60
    }

    pub fn uptime_in_hours(&self) -> usize {
        self.uptime_seconds / 3600
    }
}
