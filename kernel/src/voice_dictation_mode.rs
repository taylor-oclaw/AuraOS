extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct VoiceDictationMode {
    active: bool,
    language: String,
    dictation_buffer: Vec<String>,
}

impl VoiceDictationMode {
    pub fn new(language: &str) -> Self {
        VoiceDictationMode {
            active: false,
            language: String::from(language),
            dictation_buffer: Vec::new(),
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_language(&mut self, language: &str) {
        self.language = String::from(language);
    }

    pub fn get_language(&self) -> &String {
        &self.language
    }

    pub fn add_to_buffer(&mut self, text: &str) {
        if self.active {
            self.dictation_buffer.push(String::from(text));
        }
    }

    pub fn clear_buffer(&mut self) {
        self.dictation_buffer.clear();
    }

    pub fn get_buffer_content(&self) -> &Vec<String> {
        &self.dictation_buffer
    }
}
