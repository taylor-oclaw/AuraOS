extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ContextEmergencyDetect {
    // Example fields for demonstration purposes
    critical_processes: Vec<String>,
    system_load_threshold: u32,
    memory_usage_threshold: u32,
    disk_space_threshold: u32,
    network_traffic_threshold: u32,
}

impl ContextEmergencyDetect {
    pub fn new(
        critical_processes: Vec<String>,
        system_load_threshold: u32,
        memory_usage_threshold: u32,
        disk_space_threshold: u32,
        network_traffic_threshold: u32,
     -> Self {
        ContextEmergencyDetect {
            critical_processes,
            system_load_threshold,
            memory_usage_threshold,
            disk_space_threshold,
            network_traffic_threshold,
        }
    }

    pub fn is_critical_process_running(&self, process_name: &str) -> bool {
        self.critical_processes.contains(&String::from(process_name))
    }

    pub fn check_system_load(&self, current_load: u32) -> bool {
        current_load > self.system_load_threshold
    }

    pub fn check_memory_usage(&self, current_usage: u32) -> bool {
        current_usage > self.memory_usage_threshold
    }

    pub fn check_disk_space(&self, available_space: u32) -> bool {
        available_space < self.disk_space_threshold
    }

    pub fn check_network_traffic(&self, traffic_rate: u32) -> bool {
        traffic_rate > self.network_traffic_threshold
    }
}
