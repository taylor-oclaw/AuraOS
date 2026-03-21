extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshCountryProfileIn {
    entries: Vec<String>,
    active: bool,
}

impl MeshCountryProfileIn {
    pub fn new() -> Self { MeshCountryProfileIn { entries: Vec::new(), active: true } }
    pub fn add(&mut self, entry: &str) { self.entries.push(String::from(entry)); }
    pub fn count(&self) -> usize { self.entries.len() }
    pub fn is_active(&self) -> bool { self.active }
}
