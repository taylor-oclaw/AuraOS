extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct AgentRetryPolicy {
    max_retries: u32,
    retry_interval: u64,
    backoff_factor: f64,
    current_retry_count: u32,
    error_messages: Vec<String>,
}

impl AgentRetryPolicy {
    pub fn new(max_retries: u32, retry_interval: u64, backoff_factor: f64) -> Self {
        AgentRetryPolicy {
            max_retries,
            retry_interval,
            backoff_factor,
            current_retry_count: 0,
            error_messages: Vec::new(),
        }
    }

    pub fn should_retry(&self) -> bool {
        self.current_retry_count < self.max_retries
    }

    pub fn get_next_retry_delay(&self) -> u64 {
        (self.retry_interval as f64 * self.backoff_factor.powi(self.current_retry_count as i32)) as u64
    }

    pub fn increment_retry_count(&mut self) {
        if self.should_retry() {
            self.current_retry_count += 1;
        }
    }

    pub fn add_error_message(&mut self, message: String) {
        self.error_messages.push(message);
    }

    pub fn get_error_messages(&self) -> &Vec<String> {
        &self.error_messages
    }
}
