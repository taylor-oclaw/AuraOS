extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct TravelDocumentCheck {
    documents: Vec<String>,
}

impl TravelDocumentCheck {
    pub fn new() -> Self {
        TravelDocumentCheck {
            documents: Vec::new(),
        }
    }

    pub fn add_document(&mut self, document: String) {
        self.documents.push(document);
    }

    pub fn remove_document(&mut self, index: usize) -> Option<String> {
        if index < self.documents.len() {
            Some(self.documents.remove(index))
        } else {
            None
        }
    }

    pub fn get_document(&self, index: usize) -> Option<&String> {
        self.documents.get(index)
    }

    pub fn has_valid_documents(&self) -> bool {
        !self.documents.is_empty()
    }

    pub fn count_documents(&self) -> usize {
        self.documents.len()
    }
}
