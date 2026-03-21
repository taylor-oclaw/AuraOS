extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut doc_intelligence = DocumentIntelligence::new();
    doc_intelligence.add_document(String::from("Sample document content"));
    let summary = doc_intelligence.summarize_document(0);
    println!("Summary: {}", summary); // This line is just for demonstration, remove in final code
}

pub struct DocumentIntelligence {
    documents: Vec<String>,
}

impl DocumentIntelligence {
    pub fn new() -> Self {
        DocumentIntelligence {
            documents: Vec::new(),
        }
    }

    pub fn add_document(&mut self, content: String) {
        self.documents.push(content);
    }

    pub fn get_document_count(&self) -> usize {
        self.documents.len()
    }

    pub fn summarize_document(&self, index: usize) -> String {
        if let Some(document) = self.documents.get(index) {
            // Simple summary logic: return the first 10 words
            let mut words = document.split_whitespace();
            let mut summary = String::new();
            for _ in 0..10 {
                if let Some(word) = words.next() {
                    summary.push_str(word);
                    summary.push(' ');
                }
            }
            summary.trim_end().to_string()
        } else {
            String::from("Document not found")
        }
    }

    pub fn search_documents(&self, query: &str) -> Vec<usize> {
        let mut results = Vec::new();
        for (index, document) in self.documents.iter().enumerate() {
            if document.contains(query) {
                results.push(index);
            }
        }
        results
    }

    pub fn remove_document(&mut self, index: usize) -> bool {
        self.documents.remove(index).is_some()
    }
}
