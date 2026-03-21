extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshWorkloadIdentity {
    entries: Vec<String>,
    active: bool,
}

impl MeshWorkloadIdentity {
    pub fn new() -> Self { MeshWorkloadIdentity { entries: Vec::new(), active: true } }
    pub fn add(&mut self, entry: &str) { self.entries.push(String::from(entry)); }
    pub fn count(&self) -> usize { self.entries.len() }
    pub fn is_active(&self) -> bool { self.active }
}
