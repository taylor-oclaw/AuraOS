extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod email_phishing_detect_v2 {
    use super::*;

    pub struct EmailPhishingDetector {
        known_patterns: Vec<String>,
        suspicious_keywords: Vec<String>,
    }

    impl EmailPhishingDetector {
        pub fn new() -> Self {
            EmailPhishingDetector {
                known_patterns: vec![
                    String::from("click here"),
                    String::from("verify your account"),
                    String::from("urgent action required"),
                ],
                suspicious_keywords: vec![
                    String::from("phish"),
                    String::from("scam"),
                    String::from("malware"),
                ],
            }
        }

        pub fn add_known_pattern(&mut self, pattern: &str) {
            self.known_patterns.push(String::from(pattern));
        }

        pub fn add_suspicious_keyword(&mut self, keyword: &str) {
            self.suspicious_keywords.push(String::from(keyword));
        }

        pub fn is_phishing_email(&self, email_content: &str) -> bool {
            for pattern in &self.known_patterns {
                if email_content.contains(pattern) {
                    return true;
                }
            }
            false
        }

        pub fn contains_suspicious_keywords(&self, email_content: &str) -> bool {
            for keyword in &self.suspicious_keywords {
                if email_content.contains(keyword) {
                    return true;
                }
            }
            false
        }

        pub fn analyze_email(&self, email_content: &str) -> String {
            let mut result = String::new();
            if self.is_phishing_email(email_content) {
                result.push_str("Phishing detected based on known patterns.\n");
            }
            if self.contains_suspicious_keywords(email_content) {
                result.push_str("Suspicious keywords found in the email content.\n");
            }
            if result.is_empty() {
                result.push_str("No phishing or suspicious keywords detected.");
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::email_phishing_detect_v2::*;

    #[test]
    fn test_email_phishing_detector() {
        let mut detector = EmailPhishingDetector::new();
        assert!(!detector.is_phishing_email("Hello, this is a normal email."));
        assert!(detector.is_phishing_email("Please click here to verify your account."));

        detector.add_known_pattern("urgent");
        assert!(detector.is_phishing_email("Urgent action required immediately."));

        detector.add_suspicious_keyword("malware");
        assert!(detector.contains_suspicious_keywords("This email contains malware."));

        let analysis = detector.analyze_email("Please click here to verify your account.");
        assert_eq!(
            analysis,
            "Phishing detected based on known patterns.\nSuspicious keywords found in the email content."
        );
    }
}
