extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn tone_child_mode_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn tone_child_mode_exit() {
    // Cleanup logic for the module
}

pub struct ToneChildMode {
    name: String,
    enabled: bool,
    settings: Vec<String>,
}

impl ToneChildMode {
    pub fn new(name: &str) -> Self {
        ToneChildMode {
            name: String::from(name),
            enabled: false,
            settings: Vec::new(),
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn add_setting(&mut self, setting: &str) {
        self.settings.push(String::from(setting));
    }

    pub fn get_settings(&self) -> Vec<String> {
        self.settings.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tone_child_mode() {
        let mut mode = ToneChildMode::new("TestMode");
        assert_eq!(mode.name, "TestMode");
        assert!(!mode.is_enabled());

        mode.enable();
        assert!(mode.is_enabled());

        mode.disable();
        assert!(!mode.is_enabled());

        mode.add_setting("setting1");
        mode.add_setting("setting2");
        let settings = mode.get_settings();
        assert_eq!(settings, vec![String::from("setting1"), String::from("setting2")]);
    }
}
