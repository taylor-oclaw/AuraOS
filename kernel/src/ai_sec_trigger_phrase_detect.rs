extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct TriggerPhraseDetector {
    trigger_phrases: Vec<String>,
}

impl TriggerPhraseDetector {
    pub fn new() -> Self {
        TriggerPhraseDetector {
            trigger_phrases: Vec::new(),
        }
    }

    pub fn add_trigger_phrase(&mut self, phrase: &str) {
        self.trigger_phrases.push(String::from(phrase));
    }

    pub fn remove_trigger_phrase(&mut self, phrase: &str) -> bool {
        if let Some(index) = self.trigger_phrases.iter().position(|p| p == phrase) {
            self.trigger_phrases.remove(index);
            true
        } else {
            false
        }
    }

    pub fn contains_trigger_phrase(&self, text: &str) -> bool {
        for phrase in &self.trigger_phrases {
            if text.contains(phrase) {
                return true;
            }
        }
        false
    }

    pub fn list_trigger_phrases(&self) -> Vec<String> {
        self.trigger_phrases.clone()
    }

    pub fn clear_trigger_phrases(&mut self) {
        self.trigger_phrases.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_remove() {
        let mut detector = TriggerPhraseDetector::new();
        detector.add_trigger_phrase("hello");
        assert!(detector.contains_trigger_phrase("hello"));
        assert!(detector.remove_trigger_phrase("hello"));
        assert!(!detector.contains_trigger_phrase("hello"));
    }

    #[test]
    fn test_list_clear() {
        let mut detector = TriggerPhraseDetector::new();
        detector.add_trigger_phrase("foo");
        detector.add_trigger_phrase("bar");
        assert_eq!(detector.list_trigger_phrases().len(), 2);
        detector.clear_trigger_phrases();
        assert_eq!(detector.list_trigger_phrases().len(), 0);
    }
}
