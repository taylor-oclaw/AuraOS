extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct RuntimeResourceLimit {
    cpu_limit: u32,
    memory_limit: u64,
    disk_space_limit: u64,
    network_bandwidth_limit: u32,
    process_count_limit: u32,
}

impl RuntimeResourceLimit {
    pub fn new(cpu_limit: u32, memory_limit: u64, disk_space_limit: u64, network_bandwidth_limit: u32, process_count_limit: u32) -> Self {
        RuntimeResourceLimit {
            cpu_limit,
            memory_limit,
            disk_space_limit,
            network_bandwidth_limit,
            process_count_limit,
        }
    }

    pub fn get_cpu_limit(&self) -> u32 {
        self.cpu_limit
    }

    pub fn set_cpu_limit(&mut self, limit: u32) {
        self.cpu_limit = limit;
    }

    pub fn get_memory_limit(&self) -> u64 {
        self.memory_limit
    }

    pub fn set_memory_limit(&mut self, limit: u64) {
        self.memory_limit = limit;
    }

    pub fn get_disk_space_limit(&self) -> u64 {
        self.disk_space_limit
    }

    pub fn set_disk_space_limit(&mut self, limit: u64) {
        self.disk_space_limit = limit;
    }

    pub fn get_network_bandwidth_limit(&self) -> u32 {
        self.network_bandwidth_limit
    }

    pub fn set_network_bandwidth_limit(&mut self, limit: u32) {
        self.network_bandwidth_limit = limit;
    }

    pub fn get_process_count_limit(&self) -> u32 {
        self.process_count_limit
    }

    pub fn set_process_count_limit(&mut self, limit: u32) {
        self.process_count_limit = limit;
    }
}
