extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DocumentSearchSemantic {
    documents: Vec<String>,
}

impl DocumentSearchSemantic {
    pub fn new() -> Self {
        DocumentSearchSemantic {
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

    pub fn search_by_keyword(&self, keyword: &str) -> Vec<usize> {
        let mut results = Vec::new();
        for (index, document) in self.documents.iter().enumerate() {
            if document.contains(keyword) {
                results.push(index);
            }
        }
        results
    }

    pub fn get_document_count(&self) -> usize {
        self.documents.len()
    }

    pub fn list_documents(&self) -> Vec<String> {
        self.documents.clone()
    }
}
