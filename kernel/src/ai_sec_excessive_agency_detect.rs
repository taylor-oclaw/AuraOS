extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AIKernelModule {
    data: Vec<u8>,
    name: String,
    version: u32,
    active: bool,
    logs: Vec<String>,
}

impl AIKernelModule {
    pub fn new(name: &str, version: u32) -> Self {
        AIKernelModule {
            data: Vec::new(),
            name: String::from(name),
            version,
            active: false,
            logs: Vec::new(),
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.log("Module activated");
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.log("Module deactivated");
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn log(&mut self, message: &str) {
        let msg = String::from(message);
        self.logs.push(msg);
    }

    pub fn get_logs(&self) -> &[String] {
        &self.logs
    }
}
