extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn lang_accent_spanish_ar_init() {
    // Initialization logic for the module
}

pub extern "C" fn lang_accent_spanish_ar_exit() {
    // Cleanup logic for the module
}

pub struct SpanishAccentCorrector {
    corrections: Vec<(String, String)>,
}

impl SpanishAccentCorrector {
    pub fn new() -> Self {
        let mut corrector = SpanishAccentCorrector {
            corrections: Vec::new(),
        };
        corrector.load_corrections();
        corrector
    }

    fn load_corrections(&mut self) {
        // Example corrections, in a real scenario these would be loaded from a persistent storage or configuration
        self.corrections.push((String::from("color"), String::from("colores")));
        self.corrections.push((String::from("amigo"), String::from("amigos")));
        self.corrections.push((String::from("papa"), String::from("papas")));
        self.corrections.push((String::from("casa"), String::from("casas")));
        self.corrections.push((String::from("auto"), String::from("autos")));
    }

    pub fn correct(&self, word: &str) -> Option<String> {
        for (incorrect, correct) in &self.corrections {
            if incorrect == word {
                return Some(correct.clone());
            }
        }
        None
    }

    pub fn add_correction(&mut self, incorrect: String, correct: String) {
        self.corrections.push((incorrect, correct));
    }

    pub fn remove_correction(&mut self, incorrect: &str) -> bool {
        let pos = self.corrections.iter().position(|(i, _)| i == incorrect);
        if let Some(position) = pos {
            self.corrections.remove(position);
            true
        } else {
            false
        }
    }

    pub fn list_corrections(&self) -> Vec<(String, String)> {
        self.corrections.clone()
    }
}
