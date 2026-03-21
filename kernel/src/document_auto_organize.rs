extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut organizer = DocumentOrganizer::new();
    organizer.add_document("report.pdf");
    organizer.add_document("summary.txt");
    organizer.remove_document("summary.txt");
    organizer.list_documents();
    if organizer.contains_document("report.pdf") {
    } else {
    }
}

pub struct DocumentOrganizer {
    documents: Vec<String>,
}

impl DocumentOrganizer {
    pub fn new() -> Self {
        DocumentOrganizer {
            documents: Vec::new(),
        }
    }

    pub fn add_document(&mut self, document_name: &str) {
        if !self.contains_document(document_name) {
            self.documents.push(String::from(document_name));
        }
    }

    pub fn remove_document(&mut self, document_name: &str) {
        self.documents.retain(|doc| doc != document_name);
    }

    pub fn list_documents(&self) -> Vec<String> {
        self.documents.clone()
    }

    pub fn contains_document(&self, document_name: &str) -> bool {
        self.documents.iter().any(|doc| doc == document_name)
    }

    pub fn count_documents(&self) -> usize {
        self.documents.len()
    }
}
