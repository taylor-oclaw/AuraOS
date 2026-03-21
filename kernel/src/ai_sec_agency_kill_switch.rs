extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct KillSwitch {
    enabled: bool,
    reasons: Vec<String>,
}

impl KillSwitch {
    pub fn new() -> Self {
        KillSwitch {
            enabled: false,
            reasons: Vec::new(),
        }
    }

    pub fn enable(&mut self, reason: &str) {
        if !self.enabled {
            self.enabled = true;
            self.reasons.push(String::from(reason));
        }
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.reasons.clear();
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn add_reason(&mut self, reason: &str) {
        if self.enabled {
            self.reasons.push(String::from(reason));
        }
    }

    pub fn get_reasons(&self) -> Vec<String> {
        self.reasons.clone()
    }
}

pub extern "C" fn ai_sec_agency_kill_switch_init() -> *mut KillSwitch {
    Box::into_raw(Box::new(KillSwitch::new()))
}

pub extern "C" fn ai_sec_agency_kill_switch_enable(kill_switch: *mut KillSwitch, reason: *const u8) {
    unsafe {
        if let Some(reason_str) = core::str::from_utf8(core::slice::from_raw_parts(reason, libc::strlen(reason as *const i8))) {
            (*kill_switch).enable(reason_str);
        }
    }
}

pub extern "C" fn ai_sec_agency_kill_switch_disable(kill_switch: *mut KillSwitch) {
    unsafe {
        (*kill_switch).disable();
    }
}

pub extern "C" fn ai_sec_agency_kill_switch_is_enabled(kill_switch: *const KillSwitch) -> bool {
    unsafe { (*kill_switch).is_enabled() }
}

pub extern "C" fn ai_sec_agency_kill_switch_add_reason(kill_switch: *mut KillSwitch, reason: *const u8) {
    unsafe {
        if let Some(reason_str) = core::str::from_utf8(core::slice::from_raw_parts(reason, libc::strlen(reason as *const i8))) {
            (*kill_switch).add_reason(reason_str);
        }
    }
}

pub extern "C" fn ai_sec_agency_kill_switch_get_reasons(kill_switch: *const KillSwitch) -> Vec<String> {
    unsafe { (*kill_switch).get_reasons() }
}
