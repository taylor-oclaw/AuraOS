extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut context = ContextActivityDetect::new();
    context.start_monitoring();
    context.log_activity("System boot");
    context.report_status();
}

pub struct ContextActivityDetect {
    activities: Vec<String>,
    is_active: bool,
}

impl ContextActivityDetect {
    pub fn new() -> Self {
        ContextActivityDetect {
            activities: Vec::new(),
            is_active: false,
        }
    }

    pub fn start_monitoring(&mut self) {
        self.is_active = true;
    }

    pub fn stop_monitoring(&mut self) {
        self.is_active = false;
    }

    pub fn log_activity(&mut self, activity: &str) {
        if self.is_active {
            self.activities.push(String::from(activity));
        }
    }

    pub fn report_status(&self) {
        if self.is_active {
        } else {
        }
        for (index, activity) in self.activities.iter().enumerate() {
        }
    }

    pub fn clear_activities(&mut self) {
        self.activities.clear();
    }
}
