extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn asf_adapter_a2a_init() {
    // Initialization logic for the module
}

pub extern "C" fn asf_adapter_a2a_exit() {
    // Cleanup logic for the module
}

pub struct A2AAdapter {
    data: Vec<u8>,
    name: String,
}

impl A2AAdapter {
    pub fn new(name: &str) -> Self {
        A2AAdapter {
            data: Vec::new(),
            name: String::from(name),
        }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a2a_adapter() {
        let mut adapter = A2AAdapter::new("TestAdapter");
        assert_eq!(adapter.get_name(), "TestAdapter");

        adapter.add_data(b"Hello, World!");
        assert_eq!(adapter.get_data(), b"Hello, World!");

        adapter.clear_data();
        assert!(adapter.get_data().is_empty());

        adapter.set_name("NewName");
        assert_eq!(adapter.get_name(), "NewName");
    }
}
