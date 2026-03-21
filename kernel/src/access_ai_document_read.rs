extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let doc = AIDocument::new("Sample Document".into());
    doc.add_paragraph("This is the first paragraph.".into());
    doc.add_paragraph("This is the second paragraph.".into());

    println!("Document Title: {}", doc.get_title());
    println!("Number of Paragraphs: {}", doc.paragraph_count());

    for i in 0..doc.paragraph_count() {
        println!("Paragraph {}: {}", i + 1, doc.get_paragraph(i));
    }

    if let Some(summary) = doc.summarize() {
        println!("Summary: {}", summary);
    } else {
        println!("No summary available.");
    }
}

pub struct AIDocument {
    title: String,
    paragraphs: Vec<String>,
}

impl AIDocument {
    pub fn new(title: String) -> Self {
        AIDocument {
            title,
            paragraphs: Vec::new(),
        }
    }

    pub fn add_paragraph(&mut self, paragraph: String) {
        self.paragraphs.push(paragraph);
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn paragraph_count(&self) -> usize {
        self.paragraphs.len()
    }

    pub fn get_paragraph(&self, index: usize) -> Option<&str> {
        self.paragraphs.get(index).map(|p| p.as_str())
    }

    pub fn summarize(&self) -> Option<String> {
        if self.paragraphs.is_empty() {
            return None;
        }

        let mut summary = String::new();
        for paragraph in &self.paragraphs {
            if summary.len() + paragraph.len() > 100 {
                break;
            }
            summary.push_str(paragraph);
            summary.push(' ');
        }

        Some(summary.trim().to_string())
    }
}
