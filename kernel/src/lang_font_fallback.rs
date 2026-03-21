extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct FontFallback {
    fonts: Vec<String>,
}

impl FontFallback {
    pub fn new() -> Self {
        FontFallback { fonts: Vec::new() }
    }

    pub fn add_font(&mut self, font_name: &str) {
        self.fonts.push(String::from(font_name));
    }

    pub fn remove_font(&mut self, font_name: &str) {
        if let Some(index) = self.fonts.iter().position(|f| f == font_name) {
            self.fonts.remove(index);
        }
    }

    pub fn get_fonts(&self) -> Vec<String> {
        self.fonts.clone()
    }

    pub fn has_font(&self, font_name: &str) -> bool {
        self.fonts.contains(&String::from(font_name))
    }

    pub fn fallback_font(&self) -> Option<&String> {
        self.fonts.first()
    }
}
