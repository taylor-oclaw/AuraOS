extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn app_ocr_engine_v2_init() {
    // Initialization logic for the OCR engine module
}

#[no_mangle]
pub extern "C" fn app_ocr_engine_v2_exit() {
    // Cleanup logic for the OCR engine module
}

pub struct OCREngine {
    model: String,
    languages: Vec<String>,
    accuracy: f32,
}

impl OCREngine {
    pub fn new(model: &str, languages: &[&str], accuracy: f32) -> Self {
        OCREngine {
            model: String::from(model),
            languages: languages.iter().map(|&lang| String::from(lang)).collect(),
            accuracy,
        }
    }

    pub fn get_model(&self) -> &String {
        &self.model
    }

    pub fn set_model(&mut self, model: &str) {
        self.model = String::from(model);
    }

    pub fn get_languages(&self) -> &Vec<String> {
        &self.languages
    }

    pub fn add_language(&mut self, language: &str) {
        if !self.languages.contains(&String::from(language)) {
            self.languages.push(String::from(language));
        }
    }

    pub fn remove_language(&mut self, language: &str) {
        self.languages.retain(|lang| lang != language);
    }

    pub fn get_accuracy(&self) -> f32 {
        self.accuracy
    }

    pub fn set_accuracy(&mut self, accuracy: f32) {
        self.accuracy = accuracy;
    }
}
