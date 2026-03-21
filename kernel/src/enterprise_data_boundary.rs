extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut boundary = EnterpriseDataBoundary::new();
    boundary.add_data("data1".to_string(), vec![1, 2, 3]);
    boundary.add_data("data2".to_string(), vec![4, 5, 6]);
    boundary.print_all_keys();
    if let Some(data) = boundary.get_data("data1") {
    }
    boundary.remove_data("data2");
    boundary.print_all_keys();
    loop {}
}

pub struct EnterpriseDataBoundary {
    data_store: Vec<(String, Vec<u8>)>,
}

impl EnterpriseDataBoundary {
    pub fn new() -> Self {
        EnterpriseDataBoundary {
            data_store: Vec::new(),
        }
    }

    pub fn add_data(&mut self, key: String, value: Vec<u8>) {
        self.data_store.push((key, value));
    }

    pub fn get_data(&self, key: &str) -> Option<&Vec<u8>> {
        self.data_store.iter().find(|&&(ref k, _)| k == key).map(|(_, v)| v)
    }

    pub fn remove_data(&mut self, key: &str) {
        self.data_store.retain(|(k, _)| k != key);
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.data_store.iter().any(|(k, _)| k == key)
    }

    pub fn print_all_keys(&self) {
        for (key, _) in &self.data_store {
        }
    }
}
