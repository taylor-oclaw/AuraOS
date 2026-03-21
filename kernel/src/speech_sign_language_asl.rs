extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn speech_sign_language_asl_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn speech_sign_language_asl_exit() {
    // Cleanup logic for the module
}

pub struct SpeechSignLanguageASL {
    translations: Vec<(String, String)>,
}

impl SpeechSignLanguageASL {
    pub fn new() -> Self {
        SpeechSignLanguageASL {
            translations: Vec::new(),
        }
    }

    pub fn add_translation(&mut self, speech: &str, sign_language: &str) {
        let speech = String::from(speech);
        let sign_language = String::from(sign_language);
        self.translations.push((speech, sign_language));
    }

    pub fn get_sign_language(&self, speech: &str) -> Option<&String> {
        for (s, sl) in &self.translations {
            if s == speech {
                return Some(sl);
            }
        }
        None
    }

    pub fn remove_translation(&mut self, speech: &str) {
        self.translations.retain(|(s, _)| s != speech);
    }

    pub fn list_translations(&self) -> Vec<&String> {
        self.translations.iter().map(|(_, sl)| sl).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get_translation() {
        let mut asl = SpeechSignLanguageASL::new();
        asl.add_translation("hello", "HELLO");
        assert_eq!(asl.get_sign_language("hello"), Some(&String::from("HELLO")));
    }

    #[test]
    fn test_remove_translation() {
        let mut asl = SpeechSignLanguageASL::new();
        asl.add_translation("hello", "HELLO");
        asl.remove_translation("hello");
        assert_eq!(asl.get_sign_language("hello"), None);
    }

    #[test]
    fn test_list_translations() {
        let mut asl = SpeechSignLanguageASL::new();
        asl.add_translation("hello", "HELLO");
        asl.add_translation("world", "WORLD");
        let translations = asl.list_translations();
        assert_eq!(translations.len(), 2);
        assert!(translations.contains(&&String::from("HELLO")));
        assert!(translations.contains(&&String::from("WORLD")));
    }
}
