extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn init_module() -> i32 {
    0
}

pub extern "C" fn cleanup_module() -> i32 {
    0
}

pub struct RelationshipCommunicationFreq {
    data: Vec<(String, u32)>,
}

impl RelationshipCommunicationFreq {
    pub fn new() -> Self {
        RelationshipCommunicationFreq { data: Vec::new() }
    }

    pub fn add_communication(&mut self, name: &str, frequency: u32) {
        let name = String::from(name);
        match self.data.iter_mut().find(|(n, _)| *n == name) {
            Some((_, freq)) => *freq += frequency,
            None => self.data.push((name, frequency)),
        }
    }

    pub fn get_communication(&self, name: &str) -> Option<u32> {
        self.data.iter().find(|(n, _)| n == name).map(|(_, freq)| *freq)
    }

    pub fn remove_communication(&mut self, name: &str) {
        self.data.retain(|(n, _)| n != name);
    }

    pub fn total_communications(&self) -> u32 {
        self.data.iter().map(|(_, freq)| freq).sum()
    }

    pub fn most_frequent_communication(&self) -> Option<&String> {
        self.data.iter().max_by_key(|&(_, freq)| freq).map(|(name, _)| name)
    }
}
