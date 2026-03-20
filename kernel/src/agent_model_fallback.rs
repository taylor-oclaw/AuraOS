extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    println!("Agent Model Fallback Module Loaded");
    0
}

pub struct AgentModelFallback {
    model_name: String,
    parameters: Vec<u8>,
    version: u32,
    is_active: bool,
    error_log: String,
}

impl AgentModelFallback {
    pub fn new(model_name: &str, parameters: Vec<u8>, version: u32) -> Self {
        AgentModelFallback {
            model_name: String::from(model_name),
            parameters,
            version,
            is_active: false,
            error_log: String::new(),
        }
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn update_parameters(&mut self, new_params: Vec<u8>) {
        self.parameters = new_params;
    }

    pub fn log_error(&mut self, error_message: &str) {
        self.error_log.push_str(error_message);
        self.error_log.push('\n');
    }

    pub fn get_status(&self) -> String {
        let mut status = format!("Model Name: {}\n", self.model_name);
        status.push_str(&format!("Version: {}\n", self.version));
        status.push_str(&format!("Active: {}\n", self.is_active));
        if !self.error_log.is_empty() {
            status.push_str("Errors:\n");
            status.push_str(&self.error_log);
        }
        status
    }
}
