extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rel_throwback_generate_init() {
    // Initialization logic for the module
}

pub extern "C" fn rel_throwback_generate_exit() {
    // Cleanup logic for the module
}

pub struct RelThrowbackGenerate {
    data: Vec<u8>,
    name: String,
}

impl RelThrowbackGenerate {
    pub fn new(name: &str) -> Self {
        RelThrowbackGenerate {
            data: Vec::new(),
            name: String::from(name),
        }
    }

    pub fn add_data(&mut self, byte: u8) {
        self.data.push(byte);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn data_size(&self) -> usize {
        self.data.len()
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let generator = RelThrowbackGenerate::new("Test");
        assert_eq!(generator.get_name(), "Test");
        assert_eq!(generator.data_size(), 0);
    }

    #[test]
    fn test_add_data() {
        let mut generator = RelThrowbackGenerate::new("Test");
        generator.add_data(42);
        assert_eq!(generator.data_size(), 1);
        assert_eq!(generator.get_data()[0], 42);
    }

    #[test]
    fn test_clear_data() {
        let mut generator = RelThrowbackGenerate::new("Test");
        generator.add_data(42);
        generator.clear_data();
        assert_eq!(generator.data_size(), 0);
    }
}
