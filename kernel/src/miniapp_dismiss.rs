extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MiniAppDismiss {
    app_name: String,
    dismiss_count: u32,
    active: bool,
    reasons: Vec<String>,
}

impl MiniAppDismiss {
    pub fn new(app_name: &str) -> Self {
        MiniAppDismiss {
            app_name: String::from(app_name),
            dismiss_count: 0,
            active: true,
            reasons: Vec::new(),
        }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn add_reason(&mut self, reason: &str) {
        if self.active {
            self.reasons.push(String::from(reason));
        }
    }

    pub fn get_dismiss_count(&self) -> u32 {
        self.dismiss_count
    }

    pub fn increment_dismiss_count(&mut self) {
        if self.active {
            self.dismiss_count += 1;
        }
    }

    pub fn clear_reasons(&mut self) {
        self.reasons.clear();
    }

    pub fn get_reasons(&self) -> &Vec<String> {
        &self.reasons
    }
}
