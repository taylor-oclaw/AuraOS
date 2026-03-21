extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct SpeechSignLanguageBSL {
    vocabulary: Vec<(String, String)>, // (speech_word, sign_language_word)
}

impl SpeechSignLanguageBSL {
    pub fn new() -> Self {
        SpeechSignLanguageBSL {
            vocabulary: Vec::new(),
        }
    }

    pub fn add_translation(&mut self, speech_word: &str, sign_language_word: &str) {
        let speech = String::from(speech_word);
        let sign = String::from(sign_language_word);
        self.vocabulary.push((speech, sign));
    }

    pub fn get_sign_language(&self, speech_word: &str) -> Option<&String> {
        for (speech, sign) in &self.vocabulary {
            if speech == speech_word {
                return Some(sign);
            }
        }
        None
    }

    pub fn remove_translation(&mut self, speech_word: &str) {
        self.vocabulary.retain(|(s, _)| s != speech_word);
    }

    pub fn list_translations(&self) -> Vec<(String, String)> {
        self.vocabulary.iter().cloned().collect()
    }

    pub fn count_translations(&self) -> usize {
        self.vocabulary.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speech_sign_language_bsl() {
        let mut bsl = SpeechSignLanguageBSL::new();
        bsl.add_translation("hello", "HELLO");
        bsl.add_translation("world", "WORLD");

        assert_eq!(bsl.get_sign_language("hello"), Some(&String::from("HELLO")));
        assert_eq!(bsl.get_sign_language("world"), Some(&String::from("WORLD")));
        assert_eq!(bsl.get_sign_language("rust"), None);

        bsl.remove_translation("hello");
        assert_eq!(bsl.get_sign_language("hello"), None);

        let translations = bsl.list_translations();
        assert_eq!(translations.len(), 1);
        assert_eq!(translations[0], (String::from("world"), String::from("WORLD")));

        assert_eq!(bsl.count_translations(), 1);
    }
}
