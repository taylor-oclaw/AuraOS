extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_example() -> i32 {
    42
}

struct AuraAssistantDistractionBlock {
    blocked_sites: Vec<String>,
    allowed_sites: Vec<String>,
    user_preferences: Vec<String>,
    session_history: Vec<String>,
    system_logs: Vec<String>,
}

impl AuraAssistantDistractionBlock {
    pub fn new() -> Self {
        AuraAssistantDistractionBlock {
            blocked_sites: Vec::new(),
            allowed_sites: Vec::new(),
            user_preferences: Vec::new(),
            session_history: Vec::new(),
            system_logs: Vec::new(),
        }
    }

    pub fn block_site(&mut self, site: &str) {
        if !self.blocked_sites.contains(&site.to_string()) {
            self.blocked_sites.push(site.to_string());
            self.log_event(String::from("info"));
        }
    }

    pub fn allow_site(&mut self, site: &str) {
        if !self.allowed_sites.contains(&site.to_string()) {
            self.allowed_sites.push(site.to_string());
            self.log_event(String::from("info"));
        }
    }

    pub fn is_site_blocked(&self, site: &str) -> bool {
        self.blocked_sites.contains(&site.to_string())
    }

    pub fn log_event(&mut self, event: String) {
        self.system_logs.push(event);
    }

    pub fn get_session_history(&self) -> Vec<String> {
        self.session_history.clone()
    }
}
