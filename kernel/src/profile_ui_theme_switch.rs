extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct ProfileUIThemeSwitch {
    themes: Vec<String>,
    current_theme_index: usize,
}

impl ProfileUIThemeSwitch {
    pub fn new(themes: Vec<String>) -> Self {
        let current_theme_index = if themes.is_empty() { 0 } else { 0 };
        ProfileUIThemeSwitch {
            themes,
            current_theme_index,
        }
    }

    pub fn add_theme(&mut self, theme_name: String) {
        self.themes.push(theme_name);
    }

    pub fn remove_theme(&mut self, theme_name: &str) -> bool {
        if let Some(index) = self.themes.iter().position(|t| t == theme_name) {
            self.themes.remove(index);
            true
        } else {
            false
        }
    }

    pub fn switch_to_next_theme(&mut self) {
        if !self.themes.is_empty() {
            self.current_theme_index = (self.current_theme_index + 1) % self.themes.len();
        }
    }

    pub fn get_current_theme(&self) -> Option<&String> {
        self.themes.get(self.current_theme_index)
    }

    pub fn list_themes(&self) -> &Vec<String> {
        &self.themes
    }
}
