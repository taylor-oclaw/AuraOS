extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DocumentExpiryAlert {
    documents: Vec<(String, u32)>, // (document_name, expiry_date)
}

impl DocumentExpiryAlert {
    pub fn new() -> Self {
        DocumentExpiryAlert {
            documents: Vec::new(),
        }
    }

    pub fn add_document(&mut self, document_name: String, expiry_date: u32) {
        self.documents.push((document_name, expiry_date));
    }

    pub fn remove_document(&mut self, document_name: &str) -> bool {
        let pos = self.documents.iter().position(|(name, _)| name == document_name);
        if let Some(index) = pos {
            self.documents.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_expiry_date(&self, document_name: &str) -> Option<u32> {
        self.documents.iter()
            .find(|(name, _)| name == document_name)
            .map(|(_, expiry_date)| *expiry_date)
    }

    pub fn list_expired_documents(&self, current_date: u32) -> Vec<String> {
        self.documents.iter()
            .filter(|&(_, expiry_date)| expiry_date < current_date)
            .map(|(name, _)| name.clone())
            .collect()
    }

    pub fn count_documents(&self) -> usize {
        self.documents.len()
    }
}
