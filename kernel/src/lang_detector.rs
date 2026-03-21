extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct LanguageDetector {
    // Example data structure for language detection
    // In a real implementation, this would contain models or rules
    languages: Vec<String>,
}

impl LanguageDetector {
    pub fn new() -> Self {
        LanguageDetector {
            languages: vec![
                String::from("English"),
                String::from("Spanish"),
                String::from("French"),
                String::from("German"),
                String::from("Chinese"),
            ],
        }
    }

    pub fn detect_language(&self, text: &str) -> Option<&String> {
        // Simple heuristic-based language detection
        if text.contains("hello") || text.contains("world") {
            self.languages.iter().find(|&lang| lang == "English")
        } else if text.contains("hola") || text.contains("mundo") {
            self.languages.iter().find(|&lang| lang == "Spanish")
        } else if text.contains("bonjour") || text.contains("le monde") {
            self.languages.iter().find(|&lang| lang == "French")
        } else if text.contains("hallo") || text.contains("welt") {
            self.languages.iter().find(|&lang| lang == "German")
        } else if text.contains("你好") || text.contains("世界") {
            self.languages.iter().find(|&lang| lang == "Chinese")
        } else {
            None
        }
    }

    pub fn add_language(&mut self, language: String) {
        if !self.languages.contains(&language) {
            self.languages.push(language);
        }
    }

    pub fn remove_language(&mut self, language: &str) {
        self.languages.retain(|lang| lang != language);
    }

    pub fn list_languages(&self) -> Vec<&String> {
        self.languages.iter().collect()
    }

    pub fn is_supported(&self, language: &str) -> bool {
        self.languages.contains(&String::from(language))
    }
}
