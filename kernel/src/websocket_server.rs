extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct WebSocketServer {
    clients: Vec<String>,
}

impl WebSocketServer {
    pub fn new() -> Self {
        WebSocketServer {
            clients: Vec::new(),
        }
    }

    pub fn add_client(&mut self, client_id: &str) {
        self.clients.push(client_id.to_string());
    }

    pub fn remove_client(&mut self, client_id: &str) {
        if let Some(index) = self.clients.iter().position(|c| c == client_id) {
            self.clients.remove(index);
        }
    }

    pub fn broadcast_message(&self, message: &str) -> Vec<String> {
        self.clients
            .iter()
            .map(|client| format!("Broadcast to {}: {}", client, message))
            .collect()
    }

    pub fn get_client_count(&self) -> usize {
        self.clients.len()
    }

    pub fn list_clients(&self) -> Vec<String> {
        self.clients.clone()
    }
}
