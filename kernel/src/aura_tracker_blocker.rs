extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

pub extern "C" fn rust_ffi_exit() {
    // Cleanup the module
}

struct AuraTrackerBlocker {
    blocked_ips: Vec<String>,
    allowed_ips: Vec<String>,
    log: Vec<String>,
}

impl AuraTrackerBlocker {
    pub fn new() -> Self {
        AuraTrackerBlocker {
            blocked_ips: Vec::new(),
            allowed_ips: Vec::new(),
            log: Vec::new(),
        }
    }

    pub fn block_ip(&mut self, ip: &str) {
        if !self.allowed_ips.contains(&ip.to_string()) && !self.blocked_ips.contains(&ip.to_string()) {
            self.blocked_ips.push(ip.to_string());
            self.log_event(String::from("info"));
        }
    }

    pub fn allow_ip(&mut self, ip: &str) {
        if !self.allowed_ips.contains(&ip.to_string()) && !self.blocked_ips.contains(&ip.to_string()) {
            self.allowed_ips.push(ip.to_string());
            self.log_event(String::from("info"));
        }
    }

    pub fn is_ip_allowed(&self, ip: &str) -> bool {
        self.allowed_ips.contains(&ip.to_string())
    }

    pub fn is_ip_blocked(&self, ip: &str) -> bool {
        self.blocked_ips.contains(&ip.to_string())
    }

    pub fn get_log(&self) -> Vec<String> {
        self.log.clone()
    }

    fn log_event(&mut self, event: String) {
        self.log.push(event);
    }
}

pub extern "C" fn aura_tracker_blocker_new() -> *mut AuraTrackerBlocker {
    Box::into_raw(Box::new(AuraTrackerBlocker::new()))
}

pub extern "C" fn aura_tracker_blocker_drop(ptr: *mut AuraTrackerBlocker) {
    if !ptr.is_null() {
        unsafe { drop(Box::from_raw(ptr)) };
    }
}

pub extern "C" fn aura_tracker_blocker_block_ip(ptr: *mut AuraTrackerBlocker, ip: *const u8, len: usize) {
    if let Some(blocker) = unsafe { ptr.as_mut() } {
        let ip_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(ip, len)) };
        blocker.block_ip(ip_str);
    }
}

pub extern "C" fn aura_tracker_blocker_allow_ip(ptr: *mut AuraTrackerBlocker, ip: *const u8, len: usize) {
    if let Some(blocker) = unsafe { ptr.as_mut() } {
        let ip_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(ip, len)) };
        blocker.allow_ip(ip_str);
    }
}

pub extern "C" fn aura_tracker_blocker_is_ip_allowed(ptr: *mut AuraTrackerBlocker, ip: *const u8, len: usize) -> bool {
    if let Some(blocker) = unsafe { ptr.as_ref() } {
        let ip_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(ip, len)) };
        blocker.is_ip_allowed(ip_str)
    } else {
        false
    }
}

pub extern "C" fn aura_tracker_blocker_is_ip_blocked(ptr: *mut AuraTrackerBlocker, ip: *const u8, len: usize) -> bool {
    if let Some(blocker) = unsafe { ptr.as_ref() } {
        let ip_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(ip, len)) };
        blocker.is_ip_blocked(ip_str)
    } else {
        false
    }
}

pub extern "C" fn aura_tracker_blocker_get_log(ptr: *mut AuraTrackerBlocker) -> Vec<String> {
    if let Some(blocker) = unsafe { ptr.as_ref() } {
        blocker.get_log()
    } else {
        Vec::new()
    }
}
