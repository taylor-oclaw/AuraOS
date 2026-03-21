extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AISecAuditModelCard {
    model_name: String,
    version: String,
    description: String,
    authors: Vec<String>,
    license: String,
}

impl AISecAuditModelCard {
    pub fn new(model_name: &str, version: &str, description: &str, authors: &[&str], license: &str) -> Self {
        AISecAuditModelCard {
            model_name: String::from(model_name),
            version: String::from(version),
            description: String::from(description),
            authors: authors.iter().map(|&author| String::from(author)).collect(),
            license: String::from(license),
        }
    }

    pub fn get_model_name(&self) -> &str {
        &self.model_name
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_authors(&self) -> &[String] {
        &self.authors
    }

    pub fn get_license(&self) -> &str {
        &self.license
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_card_creation() {
        let model_card = AISecAuditModelCard::new(
            "SecureAI",
            "1.0",
            "A secure AI model for auditing.",
            &["Alice", "Bob"],
            "MIT",
        ;

        assert_eq!(model_card.get_model_name(), "SecureAI");
        assert_eq!(model_card.get_version(), "1.0");
        assert_eq!(model_card.get_description(), "A secure AI model for auditing.");
        assert_eq!(model_card.get_authors(), &[String::from("Alice"), String::from("Bob")]);
        assert_eq!(model_card.get_license(), "MIT");
    }
}
