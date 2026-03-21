extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn security_phishing_email_scan_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn security_phishing_email_scan_exit() {
    // Cleanup logic for the module
}

pub struct EmailScanner {
    keywords: Vec<String>,
    blacklisted_domains: Vec<String>,
}

impl EmailScanner {
    pub fn new(keywords: Vec<&str>, blacklisted_domains: Vec<&str>) -> Self {
        EmailScanner {
            keywords: keywords.into_iter().map(|s| s.to_string()).collect(),
            blacklisted_domains: blacklisted_domains.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn scan_email(&self, email_content: &str) -> bool {
        for keyword in &self.keywords {
            if email_content.contains(keyword) {
                return true;
            }
        }
        false
    }

    pub fn add_keyword(&mut self, keyword: &str) {
        self.keywords.push(keyword.to_string());
    }

    pub fn remove_keyword(&mut self, keyword: &str) {
        self.keywords.retain(|k| k != keyword);
    }

    pub fn is_domain_blacklisted(&self, domain: &str) -> bool {
        for blacklisted_domain in &self.blacklisted_domains {
            if domain.ends_with(blacklisted_domain) {
                return true;
            }
        }
        false
    }

    pub fn add_blacklisted_domain(&mut self, domain: &str) {
        self.blacklisted_domains.push(domain.to_string());
    }

    pub fn remove_blacklisted_domain(&mut self, domain: &str) {
        self.blacklisted_domains.retain(|d| d != domain);
    }
}
