extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct TraditionRemind {
    traditions: Vec<String>,
}

impl TraditionRemind {
    pub fn new() -> Self {
        TraditionRemind {
            traditions: Vec::new(),
        }
    }

    pub fn add_tradition(&mut self, tradition: String) {
        self.traditions.push(tradition);
    }

    pub fn remove_tradition(&mut self, index: usize) -> Option<String> {
        if index < self.traditions.len() {
            Some(self.traditions.remove(index))
        } else {
            None
        }
    }

    pub fn get_tradition(&self, index: usize) -> Option<&String> {
        self.traditions.get(index)
    }

    pub fn list_all_traditions(&self) -> &[String] {
        &self.traditions
    }

    pub fn count_traditions(&self) -> usize {
        self.traditions.len()
    }
}
