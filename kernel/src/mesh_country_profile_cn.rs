extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshCountryProfileCn {
    entries: Vec<String>,
    active: bool,
}

impl MeshCountryProfileCn {
    pub fn new() -> Self { MeshCountryProfileCn { entries: Vec::new(), active: true } }
    pub fn add(&mut self, entry: &str) { self.entries.push(String::from(entry)); }
    pub fn count(&self) -> usize { self.entries.len() }
    pub fn is_active(&self) -> bool { self.active }
}
