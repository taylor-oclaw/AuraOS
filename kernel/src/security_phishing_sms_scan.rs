extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

mod security_phishing_sms_scan {
    use super::*;

    pub struct SMSAnalyzer {
        known_phrases: Vec<String>,
        suspicious_keywords: Vec<String>,
    }

    impl SMSAnalyzer {
        pub fn new() -> Self {
            SMSAnalyzer {
                known_phrases: vec![
                    String::from("Congratulations, you've won!"),
                    String::from("Urgent action required."),
                    String::from("Your account has been compromised."),
                ],
                suspicious_keywords: vec![
                    String::from("prize"),
                    String::from("urgent"),
                    String::from("compromised"),
                    String::from("verify"),
                    String::from("click here"),
                ],
            }
        }

        pub fn add_known_phrase(&mut self, phrase: String) {
            self.known_phrases.push(phrase);
        }

        pub fn add_suspicious_keyword(&mut self, keyword: String) {
            self.suspicious_keywords.push(keyword);
        }

        pub fn is_known_phrase(&self, content: &str) -> bool {
            self.known_phrases.iter().any(|phrase| content.contains(phrase))
        }

        pub fn contains_suspicious_keywords(&self, content: &str) -> bool {
            self.suspicious_keywords
                .iter()
                .any(|keyword| content.to_lowercase().contains(keyword))
        }

        pub fn analyze_sms(&self, content: &str) -> String {
            if self.is_known_phrase(content) || self.contains_suspicious_keywords(content) {
                String::from("Suspicious SMS detected.")
            } else {
                String::from("SMS is safe.")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sms_analyzer() {
        let mut analyzer = security_phishing_sms_scan::SMSAnalyzer::new();
        analyzer.add_known_phrase(String::from("You have won a prize!"));
        analyzer.add_suspicious_keyword(String::from("prize"));

        assert_eq!(
            analyzer.analyze_sms("Congratulations, you've won!"),
            "Suspicious SMS detected."
        ;
        assert_eq!(
            analyzer.analyze_sms("You have won a prize!"),
            "Suspicious SMS detected."
        ;
        assert_eq!(
            analyzer.analyze_sms("This is a normal message."),
            "SMS is safe."
        ;
    }
}
