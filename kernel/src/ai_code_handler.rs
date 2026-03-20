extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let handler = AiCodeHandler::new();
    handler.initialize();
    loop {}
}

pub struct AiCodeHandler {
    code_storage: Vec<String>,
    active_code: Option<String>,
    error_log: Vec<String>,
}

impl AiCodeHandler {
    pub fn new() -> Self {
        AiCodeHandler {
            code_storage: Vec::new(),
            active_code: None,
            error_log: Vec::new(),
        }
    }

    pub fn add_code(&mut self, code: String) {
        self.code_storage.push(code);
    }

    pub fn activate_code(&mut self, index: usize) -> Result<(), String> {
        if let Some(code) = self.code_storage.get(index).cloned() {
            self.active_code = Some(code);
            Ok(())
        } else {
            Err(String::from("Code index out of bounds"))
        }
    }

    pub fn get_active_code(&self) -> Option<&String> {
        self.active_code.as_ref()
    }

    pub fn list_codes(&self) -> Vec<String> {
        self.code_storage.iter().cloned().collect()
    }

    pub fn log_error(&mut self, error: String) {
        self.error_log.push(error);
    }
}
