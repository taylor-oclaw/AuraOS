extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut mixture = MixtureExpertsV2::new();
    mixture.add_expert("Expert1");
    mixture.add_expert("Expert2");
    mixture.set_active_expert(0);
    mixture.update_expert("Expert1", "Updated Expert 1");
    let active_expert = mixture.get_active_expert();
}

pub struct MixtureExpertsV2 {
    experts: Vec<String>,
    active_index: usize,
}

impl MixtureExpertsV2 {
    pub fn new() -> Self {
        MixtureExpertsV2 {
            experts: Vec::new(),
            active_index: 0,
        }
    }

    pub fn add_expert(&mut self, name: &str) {
        self.experts.push(String::from(name));
    }

    pub fn remove_expert(&mut self, index: usize) -> Option<String> {
        if index < self.experts.len() {
            Some(self.experts.remove(index))
        } else {
            None
        }
    }

    pub fn set_active_expert(&mut self, index: usize) {
        if index < self.experts.len() {
            self.active_index = index;
        }
    }

    pub fn update_expert(&mut self, old_name: &str, new_name: &str) -> bool {
        for expert in self.experts.iter_mut() {
            if *expert == old_name {
                *expert = String::from(new_name);
                return true;
            }
        }
        false
    }

    pub fn get_active_expert(&self) -> String {
        if let Some(expert) = self.experts.get(self.active_index) {
            expert.clone()
        } else {
            String::from("No active expert")
        }
    }
}
