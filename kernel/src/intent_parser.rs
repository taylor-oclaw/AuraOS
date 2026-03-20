extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct IntentParser {
    intents: Vec<String>,
}

impl IntentParser {
    pub fn new() -> Self {
        IntentParser {
            intents: Vec::new(),
        }
    }

    pub fn add_intent(&mut self, intent: String) {
        self.intents.push(intent);
    }

    pub fn remove_intent(&mut self, intent: &str) -> bool {
        if let Some(index) = self.intents.iter().position(|i| i == intent) {
            self.intents.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_intents(&self) -> &[String] {
        &self.intents
    }

    pub fn has_intent(&self, intent: &str) -> bool {
        self.intents.contains(&String::from(intent))
    }

    pub fn clear_intents(&mut self) {
        self.intents.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_remove_intent() {
        let mut parser = IntentParser::new();
        parser.add_intent(String::from("start"));
        assert!(parser.has_intent("start"));
        assert!(parser.remove_intent("start"));
        assert!(!parser.has_intent("start"));
    }

    #[test]
    fn test_get_intents() {
        let mut parser = IntentParser::new();
        parser.add_intent(String::from("start"));
        parser.add_intent(String::from("stop"));
        let intents = parser.get_intents();
        assert_eq!(intents.len(), 2);
        assert!(intents.contains(&String::from("start")));
        assert!(intents.contains(&String::from("stop")));
    }

    #[test]
    fn test_clear_intents() {
        let mut parser = IntentParser::new();
        parser.add_intent(String::from("start"));
        parser.clear_intents();
        assert_eq!(parser.get_intents().len(), 0);
    }
}
