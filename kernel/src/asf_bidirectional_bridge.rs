extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AsfBidirectionalBridge {
    // Example fields for the bridge
    name: String,
    connected: bool,
    messages_sent: usize,
    messages_received: usize,
    buffer: Vec<u8>,
}

impl AsfBidirectionalBridge {
    pub fn new(name: &str) -> Self {
        AsfBidirectionalBridge {
            name: String::from(name),
            connected: false,
            messages_sent: 0,
            messages_received: 0,
            buffer: Vec::new(),
        }
    }

    pub fn connect(&mut self) {
        // Simulate connecting the bridge
        self.connected = true;
    }

    pub fn disconnect(&mut self) {
        // Simulate disconnecting the bridge
        self.connected = false;
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn send_message(&mut self, message: &[u8]) -> Result<(), &'static str> {
        if !self.connected {
            return Err("Bridge is not connected");
        }
        // Simulate sending a message
        self.buffer.extend_from_slice(message);
        self.messages_sent += 1;
        Ok(())
    }

    pub fn receive_message(&mut self) -> Option<Vec<u8>> {
        if self.buffer.is_empty() {
            return None;
        }
        let message = self.buffer.clone();
        self.buffer.clear();
        self.messages_received += 1;
        Some(message)
    }

    pub fn get_stats(&self) -> (usize, usize) {
        (self.messages_sent, self.messages_received)
    }
}
