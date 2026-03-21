extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut cool_down = CoolDownManager::new();
    cool_down.initialize();
    cool_down.update_status("System Booted");
    cool_down.log_event("Cool Down Module Initialized");
    cool_down.set_threshold(75);
    cool_down.check_temperature(80);
}

pub struct CoolDownManager {
    status: String,
    log: Vec<String>,
    threshold: u32,
}

impl CoolDownManager {
    pub fn new() -> Self {
        CoolDownManager {
            status: String::from("Idle"),
            log: Vec::new(),
            threshold: 70,
        }
    }

    pub fn initialize(&mut self) {
        self.status = String::from("Initialized");
        self.log_event("Initialization Complete");
    }

    pub fn update_status(&mut self, new_status: &str) {
        self.status = String::from(new_status);
        self.log_event(String::from("info"));
    }

    pub fn log_event(&mut self, event: String) {
        self.log.push(event);
    }

    pub fn set_threshold(&mut self, threshold: u32) {
        self.threshold = threshold;
        self.log_event(String::from("info"));
    }

    pub fn check_temperature(&mut self, temperature: u32) {
        if temperature > self.threshold {
            self.update_status("Cool Down Required");
            self.log_event(String::from("info"));
        } else {
            self.update_status("Normal Operation");
            self.log_event(String::from("info"));
        }
    }

    pub fn get_log(&self) -> &Vec<String> {
        &self.log
    }
}
