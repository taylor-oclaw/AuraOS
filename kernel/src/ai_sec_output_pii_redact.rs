extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct PiiRedactor {
    redacted: Vec<String>,
}

impl PiiRedactor {
    pub fn new() -> Self {
        PiiRedactor {
            redacted: Vec::new(),
        }
    }

    pub fn add_redaction(&mut self, pii: String) {
        self.redacted.push(pii);
    }

    pub fn remove_redaction(&mut self, index: usize) -> Option<String> {
        if index < self.redacted.len() {
            Some(self.redacted.remove(index))
        } else {
            None
        }
    }

    pub fn get_redactions(&self) -> &[String] {
        &self.redacted
    }

    pub fn contains_pii(&self, pii: &str) -> bool {
        self.redacted.iter().any(|p| p == pii)
    }

    pub fn clear_redactions(&mut self) {
        self.redacted.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pii_redactor() {
        let mut redactor = PiiRedactor::new();
        assert_eq!(redactor.get_redactions().len(), 0);

        redactor.add_redaction(String::from("123-45-6789"));
        assert_eq!(redactor.get_redactions().len(), 1);
        assert!(redactor.contains_pii("123-45-6789"));

        let removed = redactor.remove_redaction(0);
        assert_eq!(removed, Some(String::from("123-45-6789")));
        assert!(!redactor.contains_pii("123-45-6789"));
        assert_eq!(redactor.get_redactions().len(), 0);

        redactor.add_redaction(String::from("john.doe@example.com"));
        redactor.clear_redactions();
        assert_eq!(redactor.get_redactions().len(), 0);
    }
}
