extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

pub extern "C" fn rust_ffi_exit() {
    // Cleanup the module
}

pub struct AuraTelemetryControl {
    data: Vec<String>,
    enabled: bool,
}

impl AuraTelemetryControl {
    pub fn new() -> Self {
        AuraTelemetryControl {
            data: Vec::new(),
            enabled: true,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn add_data(&mut self, entry: String) {
        if self.enabled {
            self.data.push(entry);
        }
    }

    pub fn get_data(&self) -> &Vec<String> {
        &self.data
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }
}
