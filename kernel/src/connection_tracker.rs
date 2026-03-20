extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ConnectionTracker {
    connections: Vec<Connection>,
}

impl ConnectionTracker {
    pub fn new() -> Self {
        ConnectionTracker {
            connections: Vec::new(),
        }
    }

    pub fn add_connection(&mut self, connection: Connection) {
        self.connections.push(connection);
    }

    pub fn remove_connection(&mut self, id: u32) -> Option<Connection> {
        let pos = self.connections.iter().position(|c| c.id == id);
        if let Some(index) = pos {
            Some(self.connections.remove(index))
        } else {
            None
        }
    }

    pub fn get_connection(&self, id: u32) -> Option<&Connection> {
        self.connections.iter().find(|c| c.id == id)
    }

    pub fn list_connections(&self) -> &Vec<Connection> {
        &self.connections
    }

    pub fn count_connections(&self) -> usize {
        self.connections.len()
    }
}

pub struct Connection {
    pub id: u32,
    pub source_ip: String,
    pub destination_ip: String,
    pub status: String,
}

impl Connection {
    pub fn new(id: u32, source_ip: &str, destination_ip: &str, status: &str) -> Self {
        Connection {
            id,
            source_ip: String::from(source_ip),
            destination_ip: String::from(destination_ip),
            status: String::from(status),
        }
    }
}
