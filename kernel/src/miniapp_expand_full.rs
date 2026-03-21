extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut app = MiniAppExpandFull::new();
    app.initialize();
    app.process_data(vec![1, 2, 3, 4, 5]);
    let result = app.get_result();
    app.cleanup();
}

pub struct MiniAppExpandFull {
    data: Vec<u8>,
    processed_data: Vec<u8>,
    status: String,
}

impl MiniAppExpandFull {
    pub fn new() -> Self {
        MiniAppExpandFull {
            data: Vec::new(),
            processed_data: Vec::new(),
            status: String::from("Initialized"),
        }
    }

    pub fn initialize(&mut self) {
        self.status = String::from("Ready");
    }

    pub fn process_data(&mut self, input_data: Vec<u8>) {
        self.data = input_data;
        self.processed_data = self.data.iter().map(|&x| x * 2).collect();
        self.status = String::from("Processed");
    }

    pub fn get_result(&self) -> &Vec<u8> {
        &self.processed_data
    }

    pub fn reset(&mut self) {
        self.data.clear();
        self.processed_data.clear();
        self.status = String::from("Reset");
    }

    pub fn cleanup(&mut self) {
        self.reset();
        self.status = String::from("Cleaned up");
    }
}
