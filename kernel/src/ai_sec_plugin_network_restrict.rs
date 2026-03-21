extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn init_module() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() -> i32 {
    0
}

pub struct NetworkRestrict {
    allowed_ips: Vec<String>,
    blocked_ips: Vec<String>,
}

impl NetworkRestrict {
    pub fn new() -> Self {
        NetworkRestrict {
            allowed_ips: Vec::new(),
            blocked_ips: Vec::new(),
        }
    }

    pub fn add_allowed_ip(&mut self, ip: &str) {
        if !self.allowed_ips.contains(&ip.to_string()) {
            self.allowed_ips.push(ip.to_string());
        }
    }

    pub fn remove_allowed_ip(&mut self, ip: &str) {
        self.allowed_ips.retain(|i| i != ip);
    }

    pub fn add_blocked_ip(&mut self, ip: &str) {
        if !self.blocked_ips.contains(&ip.to_string()) {
            self.blocked_ips.push(ip.to_string());
        }
    }

    pub fn remove_blocked_ip(&mut self, ip: &str) {
        self.blocked_ips.retain(|i| i != ip);
    }

    pub fn is_ip_allowed(&self, ip: &str) -> bool {
        !self.blocked_ips.contains(&ip.to_string()) && (self.allowed_ips.is_empty() || self.allowed_ips.contains(&ip.to_string()))
    }
}
