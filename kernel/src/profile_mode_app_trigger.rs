extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    println!("Rust module started!");
    0
}

pub struct ProfileModeAppTrigger {
    enabled: bool,
    triggers: Vec<String>,
    logs: Vec<String>,
}

impl ProfileModeAppTrigger {
    pub fn new() -> Self {
        ProfileModeAppTrigger {
            enabled: false,
            triggers: Vec::new(),
            logs: Vec::new(),
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
        self.logs.push(String::from("Profile mode enabled"));
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.logs.push(String::from("Profile mode disabled"));
    }

    pub fn add_trigger(&mut self, trigger: String) {
        if !self.triggers.contains(&trigger) {
            self.triggers.push(trigger);
            self.logs.push(format!("Trigger added: {}", trigger));
        }
    }

    pub fn remove_trigger(&mut self, trigger: &str) {
        if let Some(index) = self.triggers.iter().position(|t| t == trigger) {
            self.triggers.remove(index);
            self.logs.push(format!("Trigger removed: {}", trigger));
        }
    }

    pub fn get_logs(&self) -> Vec<String> {
        self.logs.clone()
    }
}
