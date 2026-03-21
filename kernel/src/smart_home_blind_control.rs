extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut blind_control = SmartHomeBlindControl::new();
    blind_control.add_blind("Living Room", 100);
    blind_control.add_blind("Bedroom", 50);
    blind_control.move_blind("Living Room", 50);
    blind_control.get_blind_position("Bedroom");
    blind_control.list_all_blinds();
}

pub struct SmartHomeBlindControl {
    blinds: Vec<(String, u8)>,
}

impl SmartHomeBlindControl {
    pub fn new() -> Self {
        SmartHomeBlindControl {
            blinds: Vec::new(),
        }
    }

    pub fn add_blind(&mut self, name: &str, position: u8) {
        if position > 100 {
            return;
        }
        self.blinds.push((String::from(name), position));
    }

    pub fn move_blind(&mut self, name: &str, position: u8) {
        if position > 100 {
            return;
        }
        for blind in self.blinds.iter_mut() {
            if blind.0 == name {
                blind.1 = position;
                break;
            }
        }
    }

    pub fn get_blind_position(&self, name: &str) -> Option<u8> {
        for blind in self.blinds.iter() {
            if blind.0 == name {
                return Some(blind.1);
            }
        }
        None
    }

    pub fn list_all_blinds(&self) {
        for (name, position) in self.blinds.iter() {
            // Simulate printing to a log or console
        }
    }

    pub fn remove_blind(&mut self, name: &str) {
        self.blinds.retain(|blind| blind.0 != name);
    }
}
