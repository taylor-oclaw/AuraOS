extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

pub extern "C" fn rust_ffi_exit() {
    // Cleanup the module
}

pub struct AuraPanel {
    title: String,
    content: Vec<String>,
    is_active: bool,
}

impl AuraPanel {
    pub fn new(title: &str) -> Self {
        AuraPanel {
            title: String::from(title),
            content: Vec::new(),
            is_active: false,
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn add_content(&mut self, line: &str) {
        self.content.push(String::from(line));
    }

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn is_activated(&self) -> bool {
        self.is_active
    }

    pub fn display_content(&self) -> Vec<&String> {
        self.content.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aura_panel() {
        let mut panel = AuraPanel::new("Test Panel");
        assert_eq!(panel.get_title(), "Test Panel");

        panel.set_title("Updated Title");
        assert_eq!(panel.get_title(), "Updated Title");

        panel.add_content("Line 1");
        panel.add_content("Line 2");
        assert_eq!(panel.display_content().len(), 2);

        panel.clear_content();
        assert_eq!(panel.display_content().len(), 0);

        panel.activate();
        assert!(panel.is_activated());

        panel.deactivate();
        assert!(!panel.is_activated());
    }
}
