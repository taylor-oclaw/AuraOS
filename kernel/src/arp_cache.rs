extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ArpCache {
    entries: Vec<ArpEntry>,
}

impl ArpCache {
    pub fn new() -> Self {
        ArpCache {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, ip_address: String, mac_address: String) {
        let entry = ArpEntry {
            ip_address,
            mac_address,
        };
        self.entries.push(entry);
    }

    pub fn get_mac_by_ip(&self, ip_address: &str) -> Option<&String> {
        for entry in &self.entries {
            if entry.ip_address == ip_address {
                return Some(&entry.mac_address);
            }
        }
        None
    }

    pub fn remove_entry(&mut self, ip_address: &str) {
        self.entries.retain(|entry| entry.ip_address != ip_address);
    }

    pub fn list_entries(&self) -> Vec<&ArpEntry> {
        self.entries.iter().collect()
    }

    pub fn clear_cache(&mut self) {
        self.entries.clear();
    }
}

pub struct ArpEntry {
    ip_address: String,
    mac_address: String,
}
