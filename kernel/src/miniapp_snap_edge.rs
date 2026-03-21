extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let mut app = MiniAppSnapEdge::new();
    app.initialize();
    app.process_data(vec![1, 2, 3, 4, 5]);
    app.log_status();
    app.shutdown();
}

pub struct MiniAppSnapEdge {
    name: String,
    status: String,
    data: Vec<u8>,
}

impl MiniAppSnapEdge {
    pub fn new() -> Self {
        MiniAppSnapEdge {
            name: String::from("MiniAppSnapEdge"),
            status: String::from("Initialized"),
            data: Vec::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.status = String::from("Running");
    }

    pub fn process_data(&mut self, data: Vec<u8>) {
        self.data.extend(data);
        self.status = String::from("Processing Data");
    }

    pub fn log_status(&self) {
        // Simulate logging status
        println!("Status: {}", self.status);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn shutdown(&mut self) {
        self.status = String::from("Shutdown");
        self.data.clear();
    }
}
