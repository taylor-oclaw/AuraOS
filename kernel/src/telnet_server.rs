extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct TelnetServer {
    clients: Vec<String>,
    messages: Vec<String>,
}

impl TelnetServer {
    pub fn new() -> Self {
        TelnetServer {
            clients: Vec::new(),
            messages: Vec::new(),
        }
    }

    pub fn add_client(&mut self, client_name: &str) {
        self.clients.push(client_name.to_string());
    }

    pub fn remove_client(&mut self, client_name: &str) -> bool {
        if let Some(index) = self.clients.iter().position(|c| c == client_name) {
            self.clients.remove(index);
            true
        } else {
            false
        }
    }

    pub fn broadcast_message(&mut self, message: &str) {
        self.messages.push(message.to_string());
        for client in &self.clients {
            // Simulate sending a message to each client
            println!("Sending message to {}: {}", client, message);
        }
    }

    pub fn get_client_count(&self) -> usize {
        self.clients.len()
    }

    pub fn get_messages(&self) -> Vec<String> {
        self.messages.clone()
    }
}
