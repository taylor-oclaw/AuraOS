extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct MCPTransportWS {
    connection_status: String,
    messages: Vec<String>,
    buffer_size: usize,
}

impl MCPTransportWS {
    pub fn new(buffer_size: usize) -> Self {
        MCPTransportWS {
            connection_status: String::from("Disconnected"),
            messages: Vec::new(),
            buffer_size,
        }
    }

    pub fn connect(&mut self) {
        // Simulate a connection
        self.connection_status = String::from("Connected");
    }

    pub fn disconnect(&mut self) {
        // Simulate a disconnection
        self.connection_status = String::from("Disconnected");
    }

    pub fn is_connected(&self) -> bool {
        self.connection_status == "Connected"
    }

    pub fn send_message(&mut self, message: &str) {
        if self.is_connected() && message.len() <= self.buffer_size {
            self.messages.push(String::from(message));
        }
    }

    pub fn receive_messages(&mut self) -> Vec<String> {
        let messages = self.messages.clone();
        self.messages.clear();
        messages
    }
}
