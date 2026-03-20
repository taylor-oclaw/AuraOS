extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct WebSocket {
    url: String,
    connection_status: ConnectionStatus,
    buffer: Vec<u8>,
}

#[derive(Debug)]
enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
}

impl WebSocket {
    pub fn new(url: &str) -> Self {
        WebSocket {
            url: String::from(url),
            connection_status: ConnectionStatus::Disconnected,
            buffer: Vec::new(),
        }
    }

    pub fn connect(&mut self) {
        if self.connection_status == ConnectionStatus::Disconnected {
            // Simulate connection logic
            self.connection_status = ConnectionStatus::Connecting;
            // Assume connection is successful after some time
            self.connection_status = ConnectionStatus::Connected;
        }
    }

    pub fn disconnect(&mut self) {
        if self.connection_status == ConnectionStatus::Connected {
            // Simulate disconnection logic
            self.connection_status = ConnectionStatus::Disconnecting;
            // Assume disconnection is successful after some time
            self.connection_status = ConnectionStatus::Disconnected;
        }
    }

    pub fn send_message(&mut self, message: &str) -> Result<(), &'static str> {
        if self.connection_status != ConnectionStatus::Connected {
            return Err("WebSocket is not connected");
        }
        // Simulate sending a message
        self.buffer.extend_from_slice(message.as_bytes());
        Ok(())
    }

    pub fn receive_message(&mut self) -> Option<String> {
        if self.connection_status != ConnectionStatus::Connected || self.buffer.is_empty() {
            return None;
        }
        // Simulate receiving a message
        let message = String::from_utf8(self.buffer.clone()).ok()?;
        self.buffer.clear();
        Some(message)
    }

    pub fn is_connected(&self) -> bool {
        self.connection_status == ConnectionStatus::Connected
    }
}
