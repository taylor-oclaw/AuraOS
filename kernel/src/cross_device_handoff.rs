extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct CrossDeviceHandoff {
    data: Vec<u8>,
    device_id: u32,
    status: String,
}

impl CrossDeviceHandoff {
    pub fn new(device_id: u32) -> Self {
        CrossDeviceHandoff {
            data: Vec::new(),
            device_id,
            status: String::from("Initialized"),
        }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
        self.status = String::from("Data Added");
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
        self.status = String::from("Data Cleared");
    }

    pub fn set_status(&mut self, status: &str) {
        self.status = String::from(status);
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }
}
