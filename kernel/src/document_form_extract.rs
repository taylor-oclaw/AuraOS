extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DocumentFormExtractor {
    // Example fields for the extractor
    content: String,
    extracted_data: Vec<String>,
}

impl DocumentFormExtractor {
    pub fn new(content: &str) -> Self {
        DocumentFormExtractor {
            content: String::from(content),
            extracted_data: Vec::new(),
        }
    }

    pub fn extract_name(&mut self) -> Option<&str> {
        // Example extraction logic for a name
        let start = self.content.find("Name:")?;
        let end = self.content[start..].find('\n')?;
        let name = &self.content[start + 5..start + end];
        self.extracted_data.push(String::from(name));
        Some(name)
    }

    pub fn extract_email(&mut self) -> Option<&str> {
        // Example extraction logic for an email
        let start = self.content.find("Email:")?;
        let end = self.content[start..].find('\n')?;
        let email = &self.content[start + 6..start + end];
        self.extracted_data.push(String::from(email));
        Some(email)
    }

    pub fn extract_phone(&mut self) -> Option<&str> {
        // Example extraction logic for a phone number
        let start = self.content.find("Phone:")?;
        let end = self.content[start..].find('\n')?;
        let phone = &self.content[start + 6..start + end];
        self.extracted_data.push(String::from(phone));
        Some(phone)
    }

    pub fn extract_address(&mut self) -> Option<&str> {
        // Example extraction logic for an address
        let start = self.content.find("Address:")?;
        let end = self.content[start..].find('\n')?;
        let address = &self.content[start + 8..start + end];
        self.extracted_data.push(String::from(address));
        Some(address)
    }

    pub fn get_extracted_data(&self) -> &[String] {
        // Method to retrieve all extracted data
        &self.extracted_data
    }
}
