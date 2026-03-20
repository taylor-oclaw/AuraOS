extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut handler = AiDocumentHandler::new();
    handler.load_document("example.txt");
    handler.add_annotation(0, 5, "annotation1".to_string());
    handler.add_highlight(10, 20);
    println!("Annotations: {:?}", handler.get_annotations());
    println!("Highlights: {:?}", handler.get_highlights());
}

pub struct AiDocumentHandler {
    document_content: String,
    annotations: Vec<(usize, usize, String)>,
    highlights: Vec<(usize, usize)>,
}

impl AiDocumentHandler {
    pub fn new() -> Self {
        AiDocumentHandler {
            document_content: String::new(),
            annotations: Vec::new(),
            highlights: Vec::new(),
        }
    }

    pub fn load_document(&mut self, content: &str) {
        self.document_content = content.to_string();
    }

    pub fn add_annotation(&mut self, start: usize, end: usize, text: String) {
        if start <= end && end <= self.document_content.len() {
            self.annotations.push((start, end, text));
        }
    }

    pub fn add_highlight(&mut self, start: usize, end: usize) {
        if start <= end && end <= self.document_content.len() {
            self.highlights.push((start, end));
        }
    }

    pub fn get_annotations(&self) -> &Vec<(usize, usize, String)> {
        &self.annotations
    }

    pub fn get_highlights(&self) -> &Vec<(usize, usize)> {
        &self.highlights
    }
}
