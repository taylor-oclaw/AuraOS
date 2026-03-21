extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ContextLightAware {
    context_id: u32,
    light_level: u8,
    active: bool,
    history: Vec<u8>,
}

impl ContextLightAware {
    pub fn new(context_id: u32) -> Self {
        ContextLightAware {
            context_id,
            light_level: 0,
            active: false,
            history: Vec::new(),
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn set_light_level(&mut self, level: u8) {
        if level <= 100 {
            self.light_level = level;
            self.history.push(level);
        }
    }

    pub fn get_light_level(&self) -> u8 {
        self.light_level
    }

    pub fn get_history(&self) -> &Vec<u8> {
        &self.history
    }
}
