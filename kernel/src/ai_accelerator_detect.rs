extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AiAcceleratorDetect {
    accelerators: Vec<String>,
}

impl AiAcceleratorDetect {
    pub fn new() -> Self {
        AiAcceleratorDetect {
            accelerators: Vec::new(),
        }
    }

    pub fn add_accelerator(&mut self, name: &str) {
        self.accelerators.push(String::from(name));
    }

    pub fn remove_accelerator(&mut self, name: &str) -> bool {
        if let Some(index) = self.accelerators.iter().position(|x| x == name) {
            self.accelerators.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_accelerators(&self) -> Vec<String> {
        self.accelerators.clone()
    }

    pub fn count_accelerators(&self) -> usize {
        self.accelerators.len()
    }

    pub fn has_accelerator(&self, name: &str) -> bool {
        self.accelerators.contains(&String::from(name))
    }
}
