extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AINativeMemoryForget {
    memory: Vec<String>,
}

impl AINativeMemoryForget {
    pub fn new() -> Self {
        AINativeMemoryForget {
            memory: Vec::new(),
        }
    }

    pub fn store(&mut self, data: String) {
        self.memory.push(data);
    }

    pub fn retrieve(&self, index: usize) -> Option<&String> {
        self.memory.get(index)
    }

    pub fn forget(&mut self, index: usize) -> bool {
        if index < self.memory.len() {
            self.memory.remove(index);
            true
        } else {
            false
        }
    }

    pub fn clear_all(&mut self) {
        self.memory.clear();
    }

    pub fn list_memory(&self) -> Vec<&String> {
        self.memory.iter().collect()
    }
}