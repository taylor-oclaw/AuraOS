extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ContextSleepingDetect {
    process_list: Vec<String>,
    sleep_threshold: u32,
}

impl ContextSleepingDetect {
    pub fn new(sleep_threshold: u32) -> Self {
        ContextSleepingDetect {
            process_list: Vec::new(),
            sleep_threshold,
        }
    }

    pub fn add_process(&mut self, process_name: &str) {
        self.process_list.push(String::from(process_name));
    }

    pub fn remove_process(&mut self, process_name: &str) -> bool {
        if let Some(index) = self.process_list.iter().position(|p| p == process_name) {
            self.process_list.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_processes(&self) -> Vec<String> {
        self.process_list.clone()
    }

    pub fn is_sleeping(&self, process_name: &str) -> bool {
        // Simulate sleep detection logic
        let simulated_sleep_time = 10; // Example value
        simulated_sleep_time > self.sleep_threshold
    }

    pub fn update_sleep_threshold(&mut self, new_threshold: u32) {
        self.sleep_threshold = new_threshold;
    }
}
