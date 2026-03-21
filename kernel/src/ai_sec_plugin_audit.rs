extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut audit = AIAuditPlugin::new();
    audit.log_event(String::from("Module loaded"));
    audit.add_user(String::from("admin"), String::from("admin123"));
    audit.remove_user(String::from("admin"));
    audit.list_users();
    audit.clear_logs();
}

pub struct AIAuditPlugin {
    users: Vec<(String, String)>,
    logs: Vec<String>,
}

impl AIAuditPlugin {
    pub fn new() -> Self {
        AIAuditPlugin {
            users: Vec::new(),
            logs: Vec::new(),
        }
    }

    pub fn add_user(&mut self, username: String, password: String) {
        self.users.push((username, password));
        self.log_event(String::from("info").unwrap().0);
    }

    pub fn remove_user(&mut self, username: String) {
        if let Some(index) = self.users.iter().position(|&(ref u, _)| *u == username) {
            self.users.remove(index);
            self.log_event(String::from("info"));
        } else {
            self.log_event(String::from("info"));
        }
    }

    pub fn list_users(&self) -> Vec<String> {
        let mut user_list = Vec::new();
        for (username, _) in &self.users {
            user_list.push(username.clone());
        }
        self.log_event(String::from("Listed users"));
        user_list
    }

    pub fn log_event(&mut self, event: String) {
        self.logs.push(event);
    }

    pub fn clear_logs(&mut self) {
        self.logs.clear();
        self.log_event(String::from("Logs cleared"));
    }
}
