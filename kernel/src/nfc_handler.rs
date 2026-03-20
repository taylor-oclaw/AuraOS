extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut handler = NFCHandler::new();
    handler.initialize();
    handler.register_device("NFC123");
    handler.send_data(String::from("Hello, NFC!"));
    let received_data = handler.receive_data();
    println!("Received: {}", received_data);
    handler.unregister_device("NFC123");
}

pub struct NFCHandler {
    devices: Vec<String>,
    data_buffer: String,
}

impl NFCHandler {
    pub fn new() -> Self {
        NFCHandler {
            devices: Vec::new(),
            data_buffer: String::new(),
        }
    }

    pub fn initialize(&mut self) {
        // Initialize the NFC handler
        println!("NFC Handler initialized.");
    }

    pub fn register_device(&mut self, device_id: &str) {
        // Register a new NFC device
        self.devices.push(String::from(device_id));
        println!("Device {} registered.", device_id);
    }

    pub fn unregister_device(&mut self, device_id: &str) {
        // Unregister an NFC device
        if let Some(index) = self.devices.iter().position(|d| d == device_id) {
            self.devices.remove(index);
            println!("Device {} unregistered.", device_id);
        } else {
            println!("Device {} not found.", device_id);
        }
    }

    pub fn send_data(&mut self, data: String) {
        // Send data to an NFC device
        self.data_buffer = data;
        println!("Data sent: {}", self.data_buffer);
    }

    pub fn receive_data(&self) -> String {
        // Receive data from an NFC device
        self.data_buffer.clone()
    }
}
