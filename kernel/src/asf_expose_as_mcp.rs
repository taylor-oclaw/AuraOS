extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn asf_expose_as_mcp_init() {
    // Initialization logic for the module
}

pub extern "C" fn asf_expose_as_mcp_exit() {
    // Cleanup logic for the module
}

pub struct AINativeOSKernelModule {
    data: Vec<u8>,
    name: String,
    version: u32,
    active: bool,
}

impl AINativeOSKernelModule {
    pub fn new(name: &str, version: u32) -> Self {
        AINativeOSKernelModule {
            data: Vec::new(),
            name: String::from(name),
            version,
            active: false,
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn add_data(&mut self, data: u8) {
        self.data.push(data);
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
}
