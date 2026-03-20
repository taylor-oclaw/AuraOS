extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ProxyServer {
    host: String,
    port: u16,
    connections: Vec<String>,
}

impl ProxyServer {
    pub fn new(host: &str, port: u16) -> Self {
        ProxyServer {
            host: String::from(host),
            port,
            connections: Vec::new(),
        }
    }

    pub fn add_connection(&mut self, connection: &str) {
        self.connections.push(String::from(connection));
    }

    pub fn remove_connection(&mut self, connection: &str) -> bool {
        if let Some(index) = self.connections.iter().position(|c| c == connection) {
            self.connections.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_connections(&self) -> Vec<String> {
        self.connections.clone()
    }

    pub fn get_host(&self) -> &str {
        &self.host
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }
}
