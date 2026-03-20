extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn crucible_template_init() {
    // Initialization logic for the module
}

pub extern "C" fn crucible_template_exit() {
    // Cleanup logic for the module
}

pub struct CrucibleTemplate {
    data: Vec<u8>,
    name: String,
}

impl CrucibleTemplate {
    pub fn new(name: &str) -> Self {
        CrucibleTemplate {
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
    fn test_crucible_template() {
        let mut template = CrucibleTemplate::new("TestModule");
        assert_eq!(template.get_name(), "TestModule");

        template.add_data(42);
        assert_eq!(template.get_data(), &[42]);

        template.clear_data();
        assert!(template.get_data().is_empty());

        template.set_name("NewName");
        assert_eq!(template.get_name(), "NewName");
    }
}
