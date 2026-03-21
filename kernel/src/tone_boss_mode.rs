extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ToneBossMode {
    mode_name: String,
    effects: Vec<String>,
    active: bool,
}

impl ToneBossMode {
    pub fn new(mode_name: &str) -> Self {
        ToneBossMode {
            mode_name: String::from(mode_name),
            effects: Vec::new(),
            active: false,
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn add_effect(&mut self, effect: &str) {
        self.effects.push(String::from(effect));
    }

    pub fn remove_effect(&mut self, effect: &str) {
        self.effects.retain(|e| e != effect);
    }

    pub fn list_effects(&self) -> Vec<String> {
        self.effects.clone()
    }
}
