extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentSelfReflection {
    entries: Vec<String>,
    active: bool,
}

impl AgentSelfReflection {
    pub fn new() -> Self {
        AgentSelfReflection { entries: Vec::new(), active: true }
    }
    pub fn add(&mut self, entry: &str) { self.entries.push(String::from(entry)); }
    pub fn remove(&mut self, entry: &str) { self.entries.retain(|e| e != entry); }
    pub fn contains(&self, entry: &str) -> bool { self.entries.iter().any(|e| e == entry) }
    pub fn count(&self) -> usize { self.entries.len() }
    pub fn clear(&mut self) { self.entries.clear(); }
    pub fn is_active(&self) -> bool { self.active }
    pub fn set_active(&mut self, active: bool) { self.active = active; }
}
