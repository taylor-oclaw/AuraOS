extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentDocGenerator {
    documents: Vec<String>,
}

impl AgentDocGenerator {
    pub fn new() -> Self {
        AgentDocGenerator {
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

    pub fn list_documents(&self) -> &[String] {
        &self.documents
    }

    pub fn clear_documents(&mut self) {
        self.documents.clear();
    }
}
