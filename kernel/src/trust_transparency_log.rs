extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut log = TrustTransparencyLog::new();
    log.add_entry(String::from("System boot"));
    log.add_entry(String::from("Kernel module loaded"));

    loop {}
}

pub struct TrustTransparencyLog {
    entries: Vec<String>,
}

impl TrustTransparencyLog {
    pub fn new() -> Self {
        TrustTransparencyLog {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: String) {
        self.entries.push(entry);
    }

    pub fn get_entries(&self) -> &Vec<String> {
        &self.entries
    }

    pub fn clear_log(&mut self) {
        self.entries.clear();
    }

    pub fn log_size(&self) -> usize {
        self.entries.len()
    }
}
