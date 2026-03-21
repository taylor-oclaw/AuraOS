extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshCountryProfileUk {
    entries: Vec<String>,
    active: bool,
}

impl MeshCountryProfileUk {
    pub fn new() -> Self { MeshCountryProfileUk { entries: Vec::new(), active: true } }
    pub fn add(&mut self, entry: &str) { self.entries.push(String::from(entry)); }
    pub fn count(&self) -> usize { self.entries.len() }
    pub fn is_active(&self) -> bool { self.active }
}
