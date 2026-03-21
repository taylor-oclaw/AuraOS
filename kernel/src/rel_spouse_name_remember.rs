extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct SpouseNameRemember {
    names: Vec<String>,
}

impl SpouseNameRemember {
    pub fn new() -> Self {
        SpouseNameRemember { names: Vec::new() }
    }

    pub fn add_name(&mut self, name: String) {
        if !self.names.contains(&name) {
            self.names.push(name);
        }
    }

    pub fn remove_name(&mut self, name: &str) -> bool {
        let pos = self.names.iter().position(|n| n == name);
        if let Some(index) = pos {
            self.names.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_names(&self) -> Vec<String> {
        self.names.clone()
    }

    pub fn contains_name(&self, name: &str) -> bool {
        self.names.contains(&String::from(name))
    }

    pub fn clear_names(&mut self) {
        self.names.clear();
    }
}
