extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn security_screen_capture_guard_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn security_screen_capture_guard_exit() {
    // Cleanup logic for the module
}

pub struct SecurityScreenCaptureGuard {
    enabled: bool,
    allowed_users: Vec<String>,
    blocked_processes: Vec<String>,
    log: Vec<String>,
}

impl SecurityScreenCaptureGuard {
    pub fn new() -> Self {
        SecurityScreenCaptureGuard {
            enabled: false,
            allowed_users: Vec::new(),
            blocked_processes: Vec::new(),
            log: Vec::new(),
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
        self.log.push(String::from("Module enabled"));
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.log.push(String::from("Module disabled"));
    }

    pub fn add_allowed_user(&mut self, user: String) {
        if !self.allowed_users.contains(&user) {
            self.allowed_users.push(user);
            self.log.push(format!("User {} added to allowed list", user));
        }
    }

    pub fn remove_allowed_user(&mut self, user: &str) {
        if let Some(index) = self.allowed_users.iter().position(|u| u == user) {
            self.allowed_users.remove(index);
            self.log.push(format!("User {} removed from allowed list", user));
        }
    }

    pub fn add_blocked_process(&mut self, process: String) {
        if !self.blocked_processes.contains(&process) {
            self.blocked_processes.push(process);
            self.log.push(format!("Process {} added to blocked list", process));
        }
    }

    pub fn remove_blocked_process(&mut self, process: &str) {
        if let Some(index) = self.blocked_processes.iter().position(|p| p == process) {
            self.blocked_processes.remove(index);
            self.log.push(format!("Process {} removed from blocked list", process));
        }
    }

    pub fn check_capture_allowed(&self, user: &str, process: &str) -> bool {
        if !self.enabled {
            return true;
        }
        if self.allowed_users.contains(&user.to_string()) {
            return true;
        }
        if self.blocked_processes.contains(&process.to_string()) {
            return false;
        }
        true
    }

    pub fn get_log(&self) -> &Vec<String> {
        &self.log
    }
}
