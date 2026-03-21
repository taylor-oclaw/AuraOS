extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct HealthDarkModeAuto {
    enabled: bool,
    threshold: u32,
    current_light_level: u32,
    history: Vec<u32>,
}

impl HealthDarkModeAuto {
    pub fn new(threshold: u32) -> Self {
        HealthDarkModeAuto {
            enabled: false,
            threshold,
            current_light_level: 0,
            history: Vec::new(),
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

    pub fn update_light_level(&mut self, level: u32) {
        if self.enabled {
            self.current_light_level = level;
            self.history.push(level);
            if level < self.threshold {
                // Logic to enable dark mode
                println!("Enabling dark mode");
            } else {
                // Logic to disable dark mode
                println!("Disabling dark mode");
            }
        }
    }

    pub fn get_history(&self) -> &Vec<u32> {
        &self.history
    }
}
