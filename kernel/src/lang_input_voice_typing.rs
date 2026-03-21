extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_input_voice_typing_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn lang_input_voice_typing_exit() {
    // Cleanup logic for the module
}

pub struct VoiceTypingEngine {
    language: String,
    vocabulary: Vec<String>,
    buffer: String,
}

impl VoiceTypingEngine {
    pub fn new(language: &str) -> Self {
        VoiceTypingEngine {
            language: String::from(language),
            vocabulary: Vec::new(),
            buffer: String::new(),
        }
    }

    pub fn set_language(&mut self, language: &str) {
        self.language = String::from(language);
    }

    pub fn add_word_to_vocabulary(&mut self, word: &str) {
        if !self.vocabulary.contains(&String::from(word)) {
            self.vocabulary.push(String::from(word));
        }
    }

    pub fn recognize_speech(&mut self, input: &str) -> bool {
        // Simulate speech recognition logic
        let recognized = self.vocabulary.contains(&String::from(input));
        if recognized {
            self.buffer.push_str(input);
            self.buffer.push(' ');
        }
        recognized
    }

    pub fn get_transcription(&self) -> String {
        self.buffer.trim_end().to_string()
    }

    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voice_typing_engine() {
        let mut engine = VoiceTypingEngine::new("English");
        assert_eq!(engine.language, "English");

        engine.add_word_to_vocabulary("hello");
        engine.add_word_to_vocabulary("world");

        assert!(engine.recognize_speech("hello"));
        assert!(!engine.recognize_speech("goodbye"));

        let transcription = engine.get_transcription();
        assert_eq!(transcription, "hello");

        engine.clear_buffer();
        assert_eq!(engine.get_transcription(), "");
    }
}
