extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn init_module() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() -> i32 {
    0
}

pub struct VendorAlternativeFinder {
    vendors: Vec<String>,
    alternatives: Vec<Vec<String>>,
}

impl VendorAlternativeFinder {
    pub fn new(vendors: Vec<String>, alternatives: Vec<Vec<String>>) -> Self {
        VendorAlternativeFinder { vendors, alternatives }
    }

    pub fn add_vendor(&mut self, vendor: String) {
        self.vendors.push(vendor);
    }

    pub fn add_alternative(&mut self, vendor_index: usize, alternative: String) {
        if let Some(alternatives) = self.alternatives.get_mut(vendor_index) {
            alternatives.push(alternative);
        }
    }

    pub fn get_vendors(&self) -> &Vec<String> {
        &self.vendors
    }

    pub fn get_alternatives_for_vendor(&self, vendor_index: usize) -> Option<&Vec<String>> {
        self.alternatives.get(vendor_index)
    }

    pub fn find_best_alternative(&self, vendor_index: usize) -> Option<&String> {
        if let Some(alternatives) = self.alternatives.get(vendor_index) {
            alternatives.first()
        } else {
            None
        }
    }
}
