extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct ParentalHomeworkMode {
    enabled: bool,
    allowed_apps: Vec<String>,
    blocked_sites: Vec<String>,
    max_session_duration: u32, // in minutes
    current_session_start_time: Option<u32>, // timestamp in minutes since epoch
}

impl ParentalHomeworkMode {
    pub fn new() -> Self {
        ParentalHomeworkMode {
            enabled: false,
            allowed_apps: Vec::new(),
            blocked_sites: Vec::new(),
            max_session_duration: 60, // default to 1 hour
            current_session_start_time: None,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
        self.current_session_start_time = Some(self.get_current_time());
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.current_session_start_time = None;
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

    pub fn add_blocked_site(&mut self, site_url: String) {
        if !self.blocked_sites.contains(&site_url) {
            self.blocked_sites.push(site_url);
        }
    }

    pub fn remove_blocked_site(&mut self, site_url: &str) {
        self.blocked_sites.retain(|site| site != site_url);
    }

    pub fn set_max_session_duration(&mut self, duration_minutes: u32) {
        self.max_session_duration = duration_minutes;
    }

    pub fn get_remaining_session_time(&self) -> Option<u32> {
        if let Some(start_time) = self.current_session_start_time {
            let current_time = self.get_current_time();
            let elapsed_time = current_time - start_time;
            if elapsed_time < self.max_session_duration {
                Some(self.max_session_duration - elapsed_time)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_current_time(&self) -> u32 {
        // Placeholder for getting the current time in minutes since epoch
        // In a real kernel module, this would involve system calls or hardware access
        0
    }
}
