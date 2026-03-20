extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn asf_expose_as_a2a_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn asf_expose_as_a2a_exit() {
    // Cleanup logic for the module
}

pub struct A2AModule {
    data: Vec<u8>,
    name: String,
}

impl A2AModule {
    pub fn new(name: &str) -> Self {
        A2AModule {
            data: Vec::new(),
            name: String::from(name),
        }
    }

    pub fn add_data(&mut self, byte: u8) {
        self.data.push(byte);
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn set_name(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a2a_module() {
        let mut module = A2AModule::new("TestModule");
        assert_eq!(module.get_name(), "TestModule");

        module.add_data(42);
        assert_eq!(module.get_data(), &[42]);

        module.clear_data();
        assert!(module.get_data().is_empty());

        module.set_name("NewName");
        assert_eq!(module.get_name(), "NewName");
    }
}
