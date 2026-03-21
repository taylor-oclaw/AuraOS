extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_accent_profile_init() {
    // Initialization code for the module
}

#[no_mangle]
pub extern "C" fn lang_accent_profile_exit() {
    // Cleanup code for the module
}

pub struct LangAccentProfile {
    name: String,
    language: String,
    accent: String,
    proficiency_level: u8, // 1 to 5 scale
    phrases: Vec<String>,
}

impl LangAccentProfile {
    pub fn new(name: &str, language: &str, accent: &str, proficiency_level: u8) -> Self {
        LangAccentProfile {
            name: String::from(name),
            language: String::from(language),
            accent: String::from(accent),
            proficiency_level,
            phrases: Vec::new(),
        }
    }

    pub fn add_phrase(&mut self, phrase: &str) {
        if self.proficiency_level > 0 && self.proficiency_level <= 5 {
            self.phrases.push(String::from(phrase));
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_language(&self) -> &str {
        &self.language
    }

    pub fn get_accent(&self) -> &str {
        &self.accent
    }

    pub fn get_proficiency_level(&self) -> u8 {
        self.proficiency_level
    }

    pub fn list_phrases(&self) -> &[String] {
        &self.phrases
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lang_accent_profile() {
        let mut profile = LangAccentProfile::new("John", "English", "American", 3);
        assert_eq!(profile.get_name(), "John");
        assert_eq!(profile.get_language(), "English");
        assert_eq!(profile.get_accent(), "American");
        assert_eq!(profile.get_proficiency_level(), 3);

        profile.add_phrase("Hello, how are you?");
        assert_eq!(profile.list_phrases().len(), 1);
        assert_eq!(profile.list_phrases()[0], "Hello, how are you?");

        profile.set_proficiency_level(4);
        assert_eq!(profile.get_proficiency_level(), 4);

        profile.add_phrase("Good morning!");
        assert_eq!(profile.list_phrases().len(), 2);
        assert_eq!(profile.list_phrases()[1], "Good morning!");
    }
}
