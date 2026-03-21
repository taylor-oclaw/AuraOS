extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn health_wellness_dashboard_init() {
    // Initialization logic for the module
}

pub extern "C" fn health_wellness_dashboard_exit() {
    // Cleanup logic for the module
}

pub struct HealthWellnessDashboard {
    user_data: Vec<String>,
    metrics: Vec<u32>,
}

impl HealthWellnessDashboard {
    pub fn new() -> Self {
        HealthWellnessDashboard {
            user_data: Vec::new(),
            metrics: Vec::new(),
        }
    }

    pub fn add_user(&mut self, user_id: &str) {
        self.user_data.push(String::from(user_id));
    }

    pub fn remove_user(&mut self, user_id: &str) -> bool {
        if let Some(index) = self.user_data.iter().position(|x| x == user_id) {
            self.user_data.remove(index);
            true
        } else {
            false
        }
    }

    pub fn add_metric(&mut self, metric_value: u32) {
        self.metrics.push(metric_value);
    }

    pub fn get_user_count(&self) -> usize {
        self.user_data.len()
    }

    pub fn get_average_metric(&self) -> Option<u32> {
        if self.metrics.is_empty() {
            None
        } else {
            let total: u32 = self.metrics.iter().sum();
            Some(total / self.metrics.len() as u32)
        }
    }
}
