extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct AccessSwitch {
    enabled: bool,
    access_list: Vec<String>,
}

impl AccessSwitch {
    pub fn new() -> Self {
        AccessSwitch {
            enabled: false,
            access_list: Vec::new(),
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

    pub fn add_access(&mut self, access: &str) {
        if !self.access_list.contains(&String::from(access)) {
            self.access_list.push(String::from(access));
        }
    }

    pub fn remove_access(&mut self, access: &str) {
        self.access_list.retain(|a| a != access);
    }

    pub fn has_access(&self, access: &str) -> bool {
        self.access_list.contains(&String::from(access))
    }

    pub fn list_accesses(&self) -> Vec<String> {
        self.access_list.clone()
    }
}
