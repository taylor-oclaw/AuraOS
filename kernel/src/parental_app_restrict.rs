extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn parental_app_restrict_init() {
    // Initialization logic for the module
}

pub extern "C" fn parental_app_restrict_exit() {
    // Cleanup logic for the module
}

pub struct ParentalAppRestrict {
    allowed_apps: Vec<String>,
    blocked_apps: Vec<String>,
}

impl ParentalAppRestrict {
    pub fn new() -> Self {
        ParentalAppRestrict {
            allowed_apps: Vec::new(),
            blocked_apps: Vec::new(),
        }
    }

    pub fn add_allowed_app(&mut self, app_name: &str) {
        if !self.allowed_apps.contains(&app_name.to_string()) {
            self.allowed_apps.push(app_name.to_string());
        }
    }

    pub fn remove_allowed_app(&mut self, app_name: &str) {
        self.allowed_apps.retain(|x| x != app_name);
    }

    pub fn add_blocked_app(&mut self, app_name: &str) {
        if !self.blocked_apps.contains(&app_name.to_string()) {
            self.blocked_apps.push(app_name.to_string());
        }
    }

    pub fn remove_blocked_app(&mut self, app_name: &str) {
        self.blocked_apps.retain(|x| x != app_name);
    }

    pub fn is_app_allowed(&self, app_name: &str) -> bool {
        !self.blocked_apps.contains(&app_name.to_string()) && self.allowed_apps.contains(&app_name.to_string())
    }
}
