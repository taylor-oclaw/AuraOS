extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SocksHandler {
    connections: Vec<String>,
}

impl SocksHandler {
    pub fn new() -> Self {
        SocksHandler {
            connections: Vec::new(),
        }
    }

    pub fn add_connection(&mut self, address: &str) {
        let addr = String::from(address);
        if !self.connections.contains(&addr) {
            self.connections.push(addr);
        }
    }

    pub fn remove_connection(&mut self, address: &str) -> bool {
        let pos = self.connections.iter().position(|x| x == address);
        match pos {
            Some(index) => {
                self.connections.remove(index);
                true
            }
            None => false,
        }
    }

    pub fn list_connections(&self) -> Vec<String> {
        self.connections.clone()
    }

    pub fn has_connection(&self, address: &str) -> bool {
        self.connections.contains(&String::from(address))
    }

    pub fn clear_connections(&mut self) {
        self.connections.clear();
    }
}
