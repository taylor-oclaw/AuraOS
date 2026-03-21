extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AITrademarkDetector {
    database: Vec<String>,
}

impl AITrademarkDetector {
    pub fn new() -> Self {
        AITrademarkDetector {
            database: Vec::new(),
        }
    }

    pub fn add_trademark(&mut self, trademark: String) {
        self.database.push(trademark);
    }

    pub fn remove_trademark(&mut self, trademark: &str) -> bool {
        if let Some(index) = self.database.iter().position(|t| t == trademark) {
            self.database.remove(index);
            true
        } else {
            false
        }
    }

    pub fn contains_trademark(&self, trademark: &str) -> bool {
        self.database.contains(&trademark.to_string())
    }

    pub fn list_trademarks(&self) -> Vec<String> {
        self.database.clone()
    }

    pub fn clear_database(&mut self) {
        self.database.clear();
    }
}
