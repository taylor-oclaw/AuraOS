extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct RouteTable {
    entries: Vec<RouteEntry>,
}

impl RouteTable {
    pub fn new() -> Self {
        RouteTable {
            entries: Vec::new(),
        }
    }

    pub fn add_route(&mut self, destination: String, gateway: String, interface: String) {
        let entry = RouteEntry {
            destination,
            gateway,
            interface,
        };
        self.entries.push(entry);
    }

    pub fn remove_route(&mut self, destination: &str) -> bool {
        self.entries.retain(|entry| entry.destination != destination);
        !self.entries.iter().any(|entry| entry.destination == destination)
    }

    pub fn get_route(&self, destination: &str) -> Option<&RouteEntry> {
        self.entries.iter().find(|entry| entry.destination == destination)
    }

    pub fn list_routes(&self) -> Vec<&RouteEntry> {
        self.entries.iter().collect()
    }

    pub fn clear_routes(&mut self) {
        self.entries.clear();
    }
}

pub struct RouteEntry {
    destination: String,
    gateway: String,
    interface: String,
}
