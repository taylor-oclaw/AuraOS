extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct RogueAgentContain {
    name: String,
    capabilities: Vec<String>,
    active: bool,
    location: String,
    mission_status: String,
}

impl RogueAgentContain {
    pub fn new(name: &str, capabilities: &[&str], location: &str) -> Self {
        RogueAgentContain {
            name: String::from(name),
            capabilities: capabilities.iter().map(|&c| String::from(c)).collect(),
            active: false,
            location: String::from(location),
            mission_status: String::from("Idle"),
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.mission_status = String::from("Active");
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.mission_status = String::from("Deactivated");
    }

    pub fn update_location(&mut self, new_location: &str) {
        self.location = String::from(new_location);
    }

    pub fn add_capability(&mut self, capability: &str) {
        if !self.capabilities.contains(&String::from(capability)) {
            self.capabilities.push(String::from(capability));
        }
    }

    pub fn remove_capability(&mut self, capability: &str) {
        self.capabilities.retain(|c| c != capability);
    }

    pub fn get_status(&self) -> String {
        String::from("info")
    }
}
