extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    loop {}
}

struct Metrics {
    cpu_usage: u32,
    memory_usage: u32,
    disk_io: u32,
    network_traffic: u32,
    process_count: u32,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            cpu_usage: 0,
            memory_usage: 0,
            disk_io: 0,
            network_traffic: 0,
            process_count: 0,
        }
    }

    pub fn update_cpu_usage(&mut self, usage: u32) {
        self.cpu_usage = usage;
    }

    pub fn update_memory_usage(&mut self, usage: u32) {
        self.memory_usage = usage;
    }

    pub fn update_disk_io(&mut self, io: u32) {
        self.disk_io = io;
    }

    pub fn update_network_traffic(&mut self, traffic: u32) {
        self.network_traffic = traffic;
    }

    pub fn update_process_count(&mut self, count: u32) {
        self.process_count = count;
    }

    pub fn get_cpu_usage(&self) -> u32 {
        self.cpu_usage
    }

    pub fn get_memory_usage(&self) -> u32 {
        self.memory_usage
    }

    pub fn get_disk_io(&self) -> u32 {
        self.disk_io
    }

    pub fn get_network_traffic(&self) -> u32 {
        self.network_traffic
    }

    pub fn get_process_count(&self) -> u32 {
        self.process_count
    }
}
