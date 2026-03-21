extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_accent_custom_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn lang_accent_custom_exit() {
    // Cleanup logic for the module
}

pub struct LangAccentCustom {
    accent_name: String,
    supported_languages: Vec<String>,
    phrases: Vec<(String, String)>, // (original, accented)
}

impl LangAccentCustom {
    pub fn new(accent_name: &str, languages: &[&str]) -> Self {
        let mut supported_languages = Vec::new();
        for lang in languages {
            supported_languages.push(String::from(*lang));
        }
        LangAccentCustom {
            accent_name: String::from(accent_name),
            supported_languages,
            phrases: Vec::new(),
        }
    }

    pub fn add_phrase(&mut self, original: &str, accented: &str) {
        self.phrases.push((String::from(original), String::from(accented)));
    }

    pub fn get_accented_phrase(&self, phrase: &str) -> Option<&String> {
        for (original, accented) in &self.phrases {
            if original == phrase {
                return Some(accented);
            }
        }
        None
    }

    pub fn list_supported_languages(&self) -> &[String] {
        &self.supported_languages
    }

    pub fn get_accent_name(&self) -> &str {
        &self.accent_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lang_accent_custom() {
        let mut accent = LangAccentCustom::new("British", &["English"]);
        accent.add_phrase("hello", "hullo");
        accent.add_phrase("goodbye", "good-bye");

        assert_eq!(accent.get_accent_name(), "British");
        assert_eq!(accent.list_supported_languages(), vec![String::from("English")]);
        assert_eq!(accent.get_accented_phrase("hello"), Some(&String::from("hullo")));
        assert_eq!(accent.get_accented_phrase("goodbye"), Some(&String::from("good-bye")));
        assert_eq!(accent.get_accented_phrase("hi"), None);
    }
}
