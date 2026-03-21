extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut report = ActivityReport::new();
    report.add_activity("Meeting with team", 120);
    report.add_activity("Code review", 60);
    report.add_activity("Documentation update", 30);
    report.add_activity("Testing", 90);
    report.add_activity("Planning session", 45);

    let total_time = report.total_time();
    let longest_activity = report.longest_activity().unwrap_or(String::from("No activities"));
    let average_time = report.average_time();

    // Example of how you might interact with the kernel or other modules
    // This is a placeholder for actual kernel interaction logic
}

pub struct ActivityReport {
    activities: Vec<(String, u32)>,
}

impl ActivityReport {
    pub fn new() -> Self {
        ActivityReport {
            activities: Vec::new(),
        }
    }

    pub fn add_activity(&mut self, description: &str, duration: u32) {
        self.activities.push((String::from(description), duration));
    }

    pub fn total_time(&self) -> u32 {
        self.activities.iter().map(|(_, duration)| duration).sum()
    }

    pub fn longest_activity(&self) -> Option<String> {
        self.activities
            .iter()
            .max_by_key(|&(_, duration)| duration)
            .map(|(description, _)| description.clone())
    }

    pub fn average_time(&self) -> f32 {
        if self.activities.is_empty() {
            0.0
        } else {
            self.total_time() as f32 / self.activities.len() as f32
        }
    }

    pub fn activities_count(&self) -> usize {
        self.activities.len()
    }
}
