extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DocumentDuplicateDetect {
    documents: Vec<String>,
}

impl DocumentDuplicateDetect {
    pub fn new() -> Self {
        DocumentDuplicateDetect {
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

    pub fn document_count(&self) -> usize {
        self.documents.len()
    }

    pub fn is_duplicate(&self, document: &str) -> bool {
        self.documents.iter().any(|d| d == document)
    }
}
