extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
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
        println!("Monitoring started");
    }

    pub fn stop_monitoring(&mut self) {
        self.is_active = false;
        println!("Monitoring stopped");
    }

    pub fn log_activity(&mut self, activity: &str) {
        if self.is_active {
            self.activities.push(String::from(activity));
            println!("Activity logged: {}", activity);
        }
    }

    pub fn report_status(&self) {
        if self.is_active {
            println!("Monitoring is active");
        } else {
            println!("Monitoring is inactive");
        }
        for (index, activity) in self.activities.iter().enumerate() {
            println!("Activity {}: {}", index + 1, activity);
        }
    }

    pub fn clear_activities(&mut self) {
        self.activities.clear();
        println!("All activities cleared");
    }
}
