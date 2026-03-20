extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let dashboard = AgentUsageDashboard::new();
    dashboard.log_usage("CPU", 75);
    dashboard.log_usage("Memory", 60);
    dashboard.log_usage("Disk", 80);

    println!("Total Usage: {}", dashboard.total_usage());
    println!("Average Usage: {}", dashboard.average_usage());

    if let Some(max) = dashboard.max_usage() {
        println!("Max Usage: {}", max);
    }

    if let Some(min) = dashboard.min_usage() {
        println!("Min Usage: {}", min);
    }
}

pub struct AgentUsageDashboard {
    usages: Vec<(String, u32)>,
}

impl AgentUsageDashboard {
    pub fn new() -> Self {
        AgentUsageDashboard { usages: Vec::new() }
    }

    pub fn log_usage(&mut self, component: &str, usage: u32) {
        let component = String::from(component);
        self.usages.push((component, usage));
    }

    pub fn total_usage(&self) -> u32 {
        self.usages.iter().map(|&(_, usage)| usage).sum()
    }

    pub fn average_usage(&self) -> f32 {
        if self.usages.is_empty() {
            0.0
        } else {
            self.total_usage() as f32 / self.usages.len() as f32
        }
    }

    pub fn max_usage(&self) -> Option<u32> {
        self.usages.iter().map(|&(_, usage)| usage).max()
    }

    pub fn min_usage(&self) -> Option<u32> {
        self.usages.iter().map(|&(_, usage)| usage).min()
    }
}
