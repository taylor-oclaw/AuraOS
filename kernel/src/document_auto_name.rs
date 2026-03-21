extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DocumentAutoName {
    prefix: String,
    suffix: String,
    counter: usize,
}

impl DocumentAutoName {
    pub fn new(prefix: &str, suffix: &str) -> Self {
        DocumentAutoName {
            prefix: String::from(prefix),
            suffix: String::from(suffix),
            counter: 0,
        }
    }

    pub fn generate_name(&mut self) -> String {
        let name = format!("{}{:04}{}", self.prefix, self.counter, self.suffix);
        self.counter += 1;
        name
    }

    pub fn reset_counter(&mut self) {
        self.counter = 0;
    }

    pub fn set_prefix(&mut self, prefix: &str) {
        self.prefix = String::from(prefix);
    }

    pub fn set_suffix(&mut self, suffix: &str) {
        self.suffix = String::from(suffix);
    }

    pub fn get_current_counter(&self) -> usize {
        self.counter
    }
}
