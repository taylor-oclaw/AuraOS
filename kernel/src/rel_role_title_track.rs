extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct RoleTitleTracker {
    roles: Vec<String>,
    titles: Vec<String>,
}

impl RoleTitleTracker {
    pub fn new() -> Self {
        RoleTitleTracker {
            roles: Vec::new(),
            titles: Vec::new(),
        }
    }

    pub fn add_role(&mut self, role: &str) {
        if !self.roles.contains(&role.to_string()) {
            self.roles.push(role.to_string());
        }
    }

    pub fn add_title(&mut self, title: &str) {
        if !self.titles.contains(&title.to_string()) {
            self.titles.push(title.to_string());
        }
    }

    pub fn get_roles(&self) -> Vec<String> {
        self.roles.clone()
    }

    pub fn get_titles(&self) -> Vec<String> {
        self.titles.clone()
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(&role.to_string())
    }

    pub fn has_title(&self, title: &str) -> bool {
        self.titles.contains(&title.to_string())
    }
}
