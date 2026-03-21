extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct MemoryRestrictPlugin {
    allowed_regions: Vec<(usize, usize)>,
    restricted_regions: Vec<(usize, usize)>,
}

impl MemoryRestrictPlugin {
    pub fn new() -> Self {
        MemoryRestrictPlugin {
            allowed_regions: Vec::new(),
            restricted_regions: Vec::new(),
        }
    }

    pub fn add_allowed_region(&mut self, start: usize, end: usize) {
        self.allowed_regions.push((start, end));
    }

    pub fn add_restricted_region(&mut self, start: usize, end: usize) {
        self.restricted_regions.push((start, end));
    }

    pub fn is_address_allowed(&self, address: usize) -> bool {
        for &(start, end) in &self.allowed_regions {
            if address >= start && address <= end {
                return true;
            }
        }
        false
    }

    pub fn is_address_restricted(&self, address: usize) -> bool {
        for &(start, end) in &self.restricted_regions {
            if address >= start && address <= end {
                return true;
            }
        }
        false
    }

    pub fn check_memory_access(&self, address: usize) -> String {
        if self.is_address_allowed(address) {
            "Access allowed".to_string()
        } else if self.is_address_restricted(address) {
            "Access restricted".to_string()
        } else {
            "Unknown region".to_string()
        }
    }
}
