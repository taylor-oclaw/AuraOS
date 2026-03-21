extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct EnterpriseDashboard {
    user_sessions: Vec<String>,
    system_logs: Vec<String>,
    active_users: usize,
}

impl EnterpriseDashboard {
    pub fn new() -> Self {
        EnterpriseDashboard {
            user_sessions: Vec::new(),
            system_logs: Vec::new(),
            active_users: 0,
        }
    }

    pub fn add_user_session(&mut self, session_id: &str) {
        self.user_sessions.push(session_id.to_string());
        self.active_users += 1;
    }

    pub fn remove_user_session(&mut self, session_id: &str) -> bool {
        if let Some(index) = self.user_sessions.iter().position(|s| s == session_id) {
            self.user_sessions.remove(index);
            self.active_users -= 1;
            true
        } else {
            false
        }
    }

    pub fn log_event(&mut self, event: &str) {
        self.system_logs.push(event.to_string());
    }

    pub fn get_active_user_count(&self) -> usize {
        self.active_users
    }

    pub fn get_system_logs(&self) -> Vec<String> {
        self.system_logs.clone()
    }
}
