extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct CoworkerDirectory {
    coworkers: Vec<String>,
}

impl CoworkerDirectory {
    pub fn new() -> Self {
        CoworkerDirectory {
            coworkers: Vec::new(),
        }
    }

    pub fn add_coworker(&mut self, name: &str) {
        self.coworkers.push(String::from(name));
    }

    pub fn remove_coworker(&mut self, name: &str) -> bool {
        if let Some(index) = self.coworkers.iter().position(|n| n == name) {
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

    pub fn find_coworker(&self, name: &str) -> Option<&String> {
        self.coworkers.iter().find(|n| n == &name)
    }
}
