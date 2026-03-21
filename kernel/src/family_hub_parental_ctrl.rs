extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn family_hub_parental_ctrl_init() {
    // Initialization logic for the module
}

pub extern "C" fn family_hub_parental_ctrl_exit() {
    // Cleanup logic for the module
}

pub struct FamilyHubParentalCtrl {
    allowed_apps: Vec<String>,
    blocked_sites: Vec<String>,
    screen_time_limit: u32,
    current_screen_time: u32,
    is_active: bool,
}

impl FamilyHubParentalCtrl {
    pub fn new() -> Self {
        FamilyHubParentalCtrl {
            allowed_apps: Vec::new(),
            blocked_sites: Vec::new(),
            screen_time_limit: 0,
            current_screen_time: 0,
            is_active: false,
        }
    }

    pub fn add_allowed_app(&mut self, app_name: &str) {
        self.allowed_apps.push(app_name.to_string());
    }

    pub fn remove_allowed_app(&mut self, app_name: &str) {
        if let Some(index) = self.allowed_apps.iter().position(|x| x == app_name) {
            self.allowed_apps.remove(index);
        }
    }

    pub fn add_blocked_site(&mut self, site_url: &str) {
        self.blocked_sites.push(site_url.to_string());
    }

    pub fn remove_blocked_site(&mut self, site_url: &str) {
        if let Some(index) = self.blocked_sites.iter().position(|x| x == site_url) {
            self.blocked_sites.remove(index);
        }
    }

    pub fn set_screen_time_limit(&mut self, limit: u32) {
        self.screen_time_limit = limit;
    }

    pub fn start_session(&mut self) {
        if !self.is_active {
            self.current_screen_time = 0;
            self.is_active = true;
        }
    }

    pub fn end_session(&mut self) {
        self.is_active = false;
    }

    pub fn update_screen_time(&mut self, time_spent: u32) {
        if self.is_active {
            self.current_screen_time += time_spent;
            if self.current_screen_time >= self.screen_time_limit {
                self.end_session();
            }
        }
    }

    pub fn is_app_allowed(&self, app_name: &str) -> bool {
        self.allowed_apps.contains(&app_name.to_string())
    }

    pub fn is_site_blocked(&self, site_url: &str) -> bool {
        self.blocked_sites.contains(&site_url.to_string())
    }
}
