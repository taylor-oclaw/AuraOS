extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub struct WebSocketManager {
    connections: Vec<String>,
}

impl WebSocketManager {
    pub fn new() -> Self {
        WebSocketManager {
            connections: Vec::new(),
        }
    }

    pub fn add_connection(&mut self, connection_id: &str) {
        if !self.connections.contains(&connection_id) {
            self.connections.push(connection_id);
        }
    }

    pub fn remove_connection(&mut self, connection_id: &str) {
        self.connections.retain(|conn| conn != connection_id);
    }

    pub fn get_connections(&self) -> Vec<String> {
        self.connections.clone()
    }

    pub fn has_connection(&self, connection_id: &str) -> bool {
        self.connections.contains(&connection_id)
    }

    pub fn count_connections(&self) -> usize {
        self.connections.len()
    }
}
