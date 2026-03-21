extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct CoworkerDetector {
    coworkers: Vec<String>,
}

impl CoworkerDetector {
    pub fn new() -> Self {
        CoworkerDetector {
            coworkers: Vec::new(),
        }
    }

    pub fn add_coworker(&mut self, name: String) {
        if !self.coworkers.contains(&name) {
            self.coworkers.push(name);
        }
    }

    pub fn remove_coworker(&mut self, name: &str) -> bool {
        let position = self.coworkers.iter().position(|x| x == name);
        if let Some(index) = position {
            self.coworkers.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_coworker_count(&self) -> usize {
        self.coworkers.len()
    }

    pub fn list_coworkers(&self) -> Vec<String> {
        self.coworkers.clone()
    }

    pub fn is_coworker_present(&self, name: &str) -> bool {
        self.coworkers.contains(&String::from(name))
    }
}
