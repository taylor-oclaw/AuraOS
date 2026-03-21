extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DocumentSummaryGenerator {
    documents: Vec<String>,
}

impl DocumentSummaryGenerator {
    pub fn new() -> Self {
        DocumentSummaryGenerator {
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

    pub fn get_document_count(&self) -> usize {
        self.documents.len()
    }

    pub fn generate_summary(&self, document_index: usize) -> Option<String> {
        if let Some(document) = self.documents.get(document_index) {
            // Simple summary generation logic (for demonstration purposes)
            let words: Vec<&str> = document.split_whitespace().collect();
            let mut summary = String::new();

            for word in words.iter().take(10) { // Take first 10 words as a simple summary
                summary.push_str(word);
                summary.push(' ');
            }

            Some(summary.trim_end().to_string())
        } else {
            None
        }
    }

    pub fn list_documents(&self) -> Vec<String> {
        self.documents.iter().map(|doc| doc.clone()).collect()
    }
}
