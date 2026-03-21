extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ToneSubordinateMode {
    mode_enabled: bool,
    settings: Vec<String>,
    current_tone: String,
    history: Vec<String>,
}

impl ToneSubordinateMode {
    pub fn new() -> Self {
        ToneSubordinateMode {
            mode_enabled: false,
            settings: Vec::new(),
            current_tone: String::from("neutral"),
            history: Vec::new(),
        }
    }

    pub fn enable_mode(&mut self) {
        self.mode_enabled = true;
    }

    pub fn disable_mode(&mut self) {
        self.mode_enabled = false;
    }

    pub fn set_current_tone(&mut self, tone: &str) {
        if self.mode_enabled {
            self.current_tone = String::from(tone);
            self.history.push(String::from(tone));
        }
    }

    pub fn get_current_tone(&self) -> &String {
        &self.current_tone
    }

    pub fn add_setting(&mut self, setting: &str) {
        if self.mode_enabled {
            self.settings.push(String::from(setting));
        }
    }

    pub fn get_settings(&self) -> &Vec<String> {
        &self.settings
    }

    pub fn get_history(&self) -> &Vec<String> {
        &self.history
    }
}
