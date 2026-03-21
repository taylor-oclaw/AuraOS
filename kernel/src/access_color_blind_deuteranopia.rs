extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AccessColorBlindDeuteranopia {
    // Example field for demonstration purposes
    user_data: Vec<u8>,
}

impl AccessColorBlindDeuteranopia {
    pub fn new() -> Self {
        AccessColorBlindDeuteranopia {
            user_data: Vec::new(),
        }
    }

    pub fn add_user_data(&mut self, data: u8) {
        self.user_data.push(data);
    }

    pub fn get_user_data(&self) -> &Vec<u8> {
        &self.user_data
    }

    pub fn clear_user_data(&mut self) {
        self.user_data.clear();
    }

    pub fn has_user_data(&self) -> bool {
        !self.user_data.is_empty()
    }

    pub fn user_data_count(&self) -> usize {
        self.user_data.len()
    }
}
