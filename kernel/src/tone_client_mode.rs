extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ToneClientMode {
    // Example fields for the struct
    enabled: bool,
    mode: String,
    history: Vec<String>,
}

impl ToneClientMode {
    pub fn new() -> Self {
        ToneClientMode {
            enabled: false,
            mode: String::from("default"),
            history: Vec::new(),
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn set_mode(&mut self, mode: &str) {
        self.mode = String::from(mode);
    }

    pub fn get_mode(&self) -> &String {
        &self.mode
    }

    pub fn add_to_history(&mut self, entry: &str) {
        self.history.push(String::from(entry));
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}
