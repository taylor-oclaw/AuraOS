extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Cleanup the module
}

pub struct AuraDoNotDisturb {
    enabled: bool,
    allowed_apps: Vec<String>,
    blocked_times: Vec<(u32, u32)>, // (start_hour, end_hour)
}

impl AuraDoNotDisturb {
    pub fn new() -> Self {
        AuraDoNotDisturb {
            enabled: false,
            allowed_apps: Vec::new(),
            blocked_times: Vec::new(),
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn add_allowed_app(&mut self, app_name: String) {
        if !self.allowed_apps.contains(&app_name) {
            self.allowed_apps.push(app_name);
        }
    }

    pub fn remove_allowed_app(&mut self, app_name: &str) {
        self.allowed_apps.retain(|app| app != app_name);
    }

    pub fn add_blocked_time(&mut self, start_hour: u32, end_hour: u32) {
        if start_hour < 24 && end_hour <= 24 && start_hour < end_hour {
            self.blocked_times.push((start_hour, end_hour));
        }
    }

    pub fn remove_blocked_time(&mut self, start_hour: u32, end_hour: u32) {
        self.blocked_times.retain(|&(s, e)| s != start_hour || e != end_hour);
    }

    pub fn is_app_allowed(&self, app_name: &str) -> bool {
        self.allowed_apps.contains(app_name)
    }

    pub fn is_time_blocked(&self, current_hour: u32) -> bool {
        for &(start_hour, end_hour) in &self.blocked_times {
            if current_hour >= start_hour && current_hour < end_hour {
                return true;
            }
        }
        false
    }
}
