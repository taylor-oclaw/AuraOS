extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CoworkerManager {
    coworkers: Vec<String>,
}

impl CoworkerManager {
    pub fn new() -> Self {
        CoworkerManager {
            coworkers: Vec::new(),
        }
    }

    pub fn add_coworker(&mut self, name: String) {
        if !self.coworkers.contains(&name) {
            self.coworkers.push(name);
        }
    }

    pub fn remove_coworker(&mut self, name: &str) -> bool {
        let pos = self.coworkers.iter().position(|x| x == name);
        if let Some(index) = pos {
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

    pub fn has_coworker(&self, name: &str) -> bool {
        self.coworkers.contains(&String::from(name))
    }
}
