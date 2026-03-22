extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AIUsageAnalytics {
    user_sessions: Vec<String>,
    total_queries: u32,
    successful_queries: u32,
    failed_queries: u32,
    average_query_time: f64,
}

impl AIUsageAnalytics {
    pub fn new() -> Self {
        AIUsageAnalytics {
            user_sessions: Vec::new(),
            total_queries: 0,
            successful_queries: 0,
            failed_queries: 0,
            average_query_time: 0.0,
        }
    }

    pub fn record_session(&mut self, session_id: &str) {
        self.user_sessions.push(session_id.to_string());
    }

    pub fn log_query_result(&mut self, success: bool, query_time: f64) {
        self.total_queries += 1;
        if success {
            self.successful_queries += 1;
        } else {
            self.failed_queries += 1;
        }
        self.update_average_query_time(query_time);
    }

    fn update_average_query_time(&mut self, query_time: f64) {
        let total_time = (self.average_query_time * self.total_queries as f64) + query_time;
        self.average_query_time = total_time / self.total_queries as f64;
    }

    pub fn get_session_count(&self) -> usize {
        self.user_sessions.len()
    }

    pub fn get_success_rate(&self) -> f64 {
        if self.total_queries == 0 {
            0.0
        } else {
            (self.successful_queries as f64 / self.total_queries as f64) * 100.0
        }
    }

    pub fn get_failure_rate(&self) -> f64 {
        if self.total_queries == 0 {
            0.0
        } else {
            (self.failed_queries as f64 / self.total_queries as f64) * 100.0
        }
    }
}