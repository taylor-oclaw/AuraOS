extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn security_anomaly_login_init() {
    // Initialization logic for the module
}

pub extern "C" fn security_anomaly_login_exit() {
    // Cleanup logic for the module
}

pub struct SecurityAnomalyLogin {
    user_logs: Vec<String>,
    threshold: usize,
}

impl SecurityAnomalyLogin {
    pub fn new(threshold: usize) -> Self {
        SecurityAnomalyLogin {
            user_logs: Vec::new(),
            threshold,
        }
    }

    pub fn log_login(&mut self, username: &str) {
        self.user_logs.push(String::from(username));
    }

    pub fn get_user_logs(&self) -> &[String] {
        &self.user_logs
    }

    pub fn clear_logs(&mut self) {
        self.user_logs.clear();
    }

    pub fn detect_anomalies(&self) -> Vec<String> {
        let mut anomalies = Vec::new();
        for log in &self.user_logs {
            if self.user_logs.iter().filter(|&&l| l == *log).count() > self.threshold {
                anomalies.push(log.clone());
            }
        }
        anomalies
    }

    pub fn set_threshold(&mut self, threshold: usize) {
        self.threshold = threshold;
    }
}
