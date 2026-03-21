extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut block = AiSecIndirectInjectBlock::new();
    block.initialize();
    loop {}
}

pub struct AiSecIndirectInjectBlock {
    data: Vec<u8>,
    status: String,
    initialized: bool,
}

impl AiSecIndirectInjectBlock {
    pub fn new() -> Self {
        AiSecIndirectInjectBlock {
            data: Vec::new(),
            status: String::from("Uninitialized"),
            initialized: false,
        }
    }

    pub fn initialize(&mut self) {
        if !self.initialized {
            self.data = vec![0; 1024]; // Allocate 1KB of memory
            self.status = String::from("Initialized");
            self.initialized = true;
        }
    }

    pub fn inject_data(&mut self, data: &[u8]) -> bool {
        if self.initialized && data.len() <= self.data.len() {
            self.data[..data.len()].copy_from_slice(data);
            return true;
        }
        false
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }

    pub fn clear_data(&mut self) {
        if self.initialized {
            self.data.fill(0);
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}
