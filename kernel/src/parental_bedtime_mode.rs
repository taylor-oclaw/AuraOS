extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ParentalBedtimeMode {
    enabled: bool,
    bedtime_start_hour: u8,
    bedtime_end_hour: u8,
    allowed_apps: Vec<String>,
    blocked_ips: Vec<String>,
}

impl ParentalBedtimeMode {
    pub fn new(start_hour: u8, end_hour: u8) -> Self {
        ParentalBedtimeMode {
            enabled: false,
            bedtime_start_hour: start_hour,
            bedtime_end_hour: end_hour,
            allowed_apps: Vec::new(),
            blocked_ips: Vec::new(),
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

    pub fn add_allowed_app(&mut self, app_name: &str) {
        if !self.allowed_apps.contains(&app_name.to_string()) {
            self.allowed_apps.push(app_name.to_string());
        }
    }

    pub fn remove_allowed_app(&mut self, app_name: &str) {
        self.allowed_apps.retain(|x| x != app_name);
    }

    pub fn add_blocked_ip(&mut self, ip_address: &str) {
        if !self.blocked_ips.contains(&ip_address.to_string()) {
            self.blocked_ips.push(ip_address.to_string());
        }
    }

    pub fn remove_blocked_ip(&mut self, ip_address: &str) {
        self.blocked_ips.retain(|x| x != ip_address);
    }

    pub fn is_app_allowed(&self, app_name: &str) -> bool {
        self.allowed_apps.contains(&app_name.to_string())
    }

    pub fn is_ip_blocked(&self, ip_address: &str) -> bool {
        self.blocked_ips.contains(&ip_address.to_string())
    }
}
