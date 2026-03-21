extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn speech_strained_voice_init() {
    // Initialization logic for the module
}

pub extern "C" fn speech_strained_voice_exit() {
    // Cleanup logic for the module
}

pub struct SpeechStrainedVoice {
    phrases: Vec<String>,
    current_phrase_index: usize,
}

impl SpeechStrainedVoice {
    pub fn new(phrases: Vec<&str>) -> Self {
        let phrase_strings: Vec<String> = phrases.into_iter().map(String::from).collect();
        SpeechStrainedVoice {
            phrases: phrase_strings,
            current_phrase_index: 0,
        }
    }

    pub fn add_phrase(&mut self, phrase: &str) {
        self.phrases.push(String::from(phrase));
    }

    pub fn remove_phrase(&mut self, index: usize) -> Option<String> {
        if index < self.phrases.len() {
            Some(self.phrases.remove(index))
        } else {
            None
        }
    }

    pub fn get_current_phrase(&self) -> &str {
        if let Some(phrase) = self.phrases.get(self.current_phrase_index) {
            phrase.as_str()
        } else {
            ""
        }
    }

    pub fn next_phrase(&mut self) -> &str {
        if self.current_phrase_index < self.phrases.len() - 1 {
            self.current_phrase_index += 1;
        }
        self.get_current_phrase()
    }

    pub fn previous_phrase(&mut self) -> &str {
        if self.current_phrase_index > 0 {
            self.current_phrase_index -= 1;
        }
        self.get_current_phrase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speech_strained_voice() {
        let mut voice = SpeechStrainedVoice::new(vec!["Hello", "World"]);
        assert_eq!(voice.get_current_phrase(), "Hello");
        assert_eq!(voice.next_phrase(), "World");
        assert_eq!(voice.previous_phrase(), "Hello");
        voice.add_phrase("Test");
        assert_eq!(voice.get_current_phrase(), "Hello");
        assert_eq!(voice.next_phrase(), "World");
        assert_eq!(voice.next_phrase(), "Test");
        assert_eq!(voice.remove_phrase(1), Some(String::from("World")));
        assert_eq!(voice.get_current_phrase(), "Hello");
        assert_eq!(voice.next_phrase(), "Test");
    }
}
