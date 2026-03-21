extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct VoiceAppSpecificVocab {
    vocab: Vec<String>,
}

impl VoiceAppSpecificVocab {
    pub fn new() -> Self {
        VoiceAppSpecificVocab {
            vocab: Vec::new(),
        }
    }

    pub fn add_word(&mut self, word: &str) {
        self.vocab.push(String::from(word));
    }

    pub fn remove_word(&mut self, word: &str) {
        if let Some(index) = self.vocab.iter().position(|w| w == word) {
            self.vocab.remove(index);
        }
    }

    pub fn contains_word(&self, word: &str) -> bool {
        self.vocab.contains(&String::from(word))
    }

    pub fn get_vocab_size(&self) -> usize {
        self.vocab.len()
    }

    pub fn list_words(&self) -> Vec<String> {
        self.vocab.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voice_app_specific_vocab() {
        let mut vocab = VoiceAppSpecificVocab::new();
        assert_eq!(vocab.get_vocab_size(), 0);

        vocab.add_word("hello");
        assert_eq!(vocab.get_vocab_size(), 1);
        assert!(vocab.contains_word("hello"));

        vocab.add_word("world");
        assert_eq!(vocab.get_vocab_size(), 2);
        assert!(vocab.contains_word("world"));

        vocab.remove_word("hello");
        assert_eq!(vocab.get_vocab_size(), 1);
        assert!(!vocab.contains_word("hello"));

        let words = vocab.list_words();
        assert_eq!(words, vec![String::from("world")]);
    }
}
