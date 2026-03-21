extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AccessColorBlindProtanopia {
    // Example data structure to hold color information
    colors: Vec<String>,
}

impl AccessColorBlindProtanopia {
    pub fn new() -> Self {
        AccessColorBlindProtanopia {
            colors: Vec::new(),
        }
    }

    pub fn add_color(&mut self, color: String) {
        self.colors.push(color);
    }

    pub fn remove_color(&mut self, index: usize) -> Option<String> {
        if index < self.colors.len() {
            Some(self.colors.remove(index))
        } else {
            None
        }
    }

    pub fn get_colors(&self) -> &Vec<String> {
        &self.colors
    }

    pub fn is_color_present(&self, color: &str) -> bool {
        self.colors.iter().any(|c| c == color)
    }

    pub fn count_colors(&self) -> usize {
        self.colors.len()
    }
}
