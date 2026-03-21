extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SecurityPhishingDetect {
    // Example fields for the phishing detection module
    known_phishing_domains: Vec<String>,
    suspicious_keywords: Vec<String>,
}

impl SecurityPhishingDetect {
    pub fn new() -> Self {
        SecurityPhishingDetect {
            known_phishing_domains: Vec::new(),
            suspicious_keywords: Vec::new(),
        }
    }

    pub fn add_known_phishing_domain(&mut self, domain: String) {
        self.known_phishing_domains.push(domain);
    }

    pub fn add_suspicious_keyword(&mut self, keyword: String) {
        self.suspicious_keywords.push(keyword);
    }

    pub fn is_phishing_domain(&self, domain: &str) -> bool {
        self.known_phishing_domains.iter().any(|d| d == domain)
    }

    pub fn contains_suspicious_keyword(&self, text: &str) -> bool {
        self.suspicious_keywords.iter().any(|k| text.contains(k))
    }

    pub fn analyze_url(&self, url: &str) -> bool {
        // Simple URL analysis logic
        let domain = match url.split_once("://") {
            Some((_, rest)) => rest.split('/').next().unwrap_or(""),
            None => "",
        };

        self.is_phishing_domain(domain) || self.contains_suspicious_keyword(url)
    }
}
