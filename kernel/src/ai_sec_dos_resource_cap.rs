extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    loop {}
}

pub struct ResourceCap {
    max_memory: usize,
    max_cpu_time: u64,
    allowed_processes: Vec<String>,
    current_memory_usage: usize,
    current_cpu_time_usage: u64,
}

impl ResourceCap {
    pub fn new(max_memory: usize, max_cpu_time: u64) -> Self {
        ResourceCap {
            max_memory,
            max_cpu_time,
            allowed_processes: Vec::new(),
            current_memory_usage: 0,
            current_cpu_time_usage: 0,
        }
    }

    pub fn add_allowed_process(&mut self, process_name: String) {
        if !self.allowed_processes.contains(&process_name) {
            self.allowed_processes.push(process_name);
        }
    }

    pub fn remove_allowed_process(&mut self, process_name: &str) {
        self.allowed_processes.retain(|p| p != process_name);
    }

    pub fn is_process_allowed(&self, process_name: &str) -> bool {
        self.allowed_processes.contains(&String::from(process_name))
    }

    pub fn allocate_memory(&mut self, amount: usize) -> bool {
        if self.current_memory_usage + amount <= self.max_memory {
            self.current_memory_usage += amount;
            true
        } else {
            false
        }
    }

    pub fn release_memory(&mut self, amount: usize) {
        if amount <= self.current_memory_usage {
            self.current_memory_usage -= amount;
        }
    }

    pub fn allocate_cpu_time(&mut self, time: u64) -> bool {
        if self.current_cpu_time_usage + time <= self.max_cpu_time {
            self.current_cpu_time_usage += time;
            true
        } else {
            false
        }
    }

    pub fn release_cpu_time(&mut self, time: u64) {
        if time <= self.current_cpu_time_usage {
            self.current_cpu_time_usage -= time;
        }
    }

    pub fn get_current_memory_usage(&self) -> usize {
        self.current_memory_usage
    }

    pub fn get_current_cpu_time_usage(&self) -> u64 {
        self.current_cpu_time_usage
    }
}
