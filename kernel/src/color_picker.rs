extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ColorPicker {
    colors: Vec<String>,
}

impl ColorPicker {
    pub fn new() -> Self {
        ColorPicker {
            colors: Vec::new(),
        }
    }

    pub fn add_color(&mut self, color: String) {
        if !self.colors.contains(&color) {
            self.colors.push(color);
        }
    }

    pub fn remove_color(&mut self, color: &str) -> bool {
        let index = self.colors.iter().position(|c| c == color);
        match index {
            Some(i) => {
                self.colors.remove(i);
                true
            }
            None => false,
        }
    }

    pub fn get_colors(&self) -> Vec<String> {
        self.colors.clone()
    }

    pub fn has_color(&self, color: &str) -> bool {
        self.colors.contains(&String::from(color))
    }

    pub fn count_colors(&self) -> usize {
        self.colors.len()
    }
}
