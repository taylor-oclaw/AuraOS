extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct CreditCardDetector {
    // Example fields for a credit card detector module
    rules: Vec<String>,
    detected_cards: Vec<String>,
}

impl CreditCardDetector {
    pub fn new() -> Self {
        CreditCardDetector {
            rules: Vec::new(),
            detected_cards: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule: String) {
        self.rules.push(rule);
    }

    pub fn detect_card(&mut self, card_number: &str) -> bool {
        for rule in &self.rules {
            if Self::matches(card_number, rule) {
                self.detected_cards.push(card_number.to_string());
                return true;
            }
        }
        false
    }

    fn matches(card_number: &str, rule: &str) -> bool {
        // Simple pattern matching logic for demonstration purposes
        card_number.contains(rule)
    }

    pub fn get_detected_cards(&self) -> &[String] {
        &self.detected_cards
    }

    pub fn clear_detected_cards(&mut self) {
        self.detected_cards.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credit_card_detector() {
        let mut detector = CreditCardDetector::new();
        detector.add_rule(String::from("1234"));
        assert!(!detector.detect_card("567890"));
        assert!(detector.detect_card("1234567890"));
        assert_eq!(detector.get_detected_cards().len(), 1);
        detector.clear_detected_cards();
        assert_eq!(detector.get_detected_cards().len(), 0);
    }
}
