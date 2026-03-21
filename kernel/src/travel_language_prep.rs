extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut prep = TravelLanguagePrep::new();
    prep.add_language("English");
    prep.add_language("Spanish");
    prep.add_language("French");
    prep.remove_language("German");
    prep.list_languages();

    loop {}
}

pub struct TravelLanguagePrep {
    languages: Vec<String>,
}

impl TravelLanguagePrep {
    pub fn new() -> Self {
        TravelLanguagePrep {
            languages: Vec::new(),
        }
    }

    pub fn add_language(&mut self, language: &str) {
        if !self.languages.contains(&String::from(language)) {
            self.languages.push(String::from(language));
        }
    }

    pub fn remove_language(&mut self, language: &str) {
        self.languages.retain(|l| l != language);
    }

    pub fn list_languages(&self) -> Vec<String> {
        self.languages.clone()
    }

    pub fn contains_language(&self, language: &str) -> bool {
        self.languages.contains(&String::from(language))
    }

    pub fn count_languages(&self) -> usize {
        self.languages.len()
    }
}
