extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Cleanup the module
}

pub struct AuraTelemetryControl {
    data: Vec<String>,
    threshold: u32,
}

impl AuraTelemetryControl {
    pub fn new(threshold: u32) -> Self {
        AuraTelemetryControl {
            data: Vec::new(),
            threshold,
        }
    }

    pub fn add_data(&mut self, value: &str) {
        if value.parse::<u32>().unwrap_or(0) > self.threshold {
            self.data.push(value.to_string());
        }
    }

    pub fn get_data(&self) -> &[String] {
        &self.data
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn set_threshold(&mut self, threshold: u32) {
        self.threshold = threshold;
    }

    pub fn above_threshold_count(&self) -> usize {
        self.data.len()
    }
}