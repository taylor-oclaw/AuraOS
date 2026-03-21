extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct DocumentLegalHighlight {
    content: String,
    highlights: Vec<String>,
}

impl DocumentLegalHighlight {
    pub fn new(content: &str) -> Self {
        DocumentLegalHighlight {
            content: String::from(content),
            highlights: Vec::new(),
        }
    }

    pub fn add_highlight(&mut self, highlight: &str) {
        if !self.highlights.contains(&String::from(highlight)) {
            self.highlights.push(String::from(highlight));
        }
    }

    pub fn remove_highlight(&mut self, highlight: &str) {
        if let Some(index) = self.highlights.iter().position(|h| h == highlight) {
            self.highlights.remove(index);
        }
    }

    pub fn get_highlights(&self) -> Vec<String> {
        self.highlights.clone()
    }

    pub fn contains_highlight(&self, highlight: &str) -> bool {
        self.highlights.contains(&String::from(highlight))
    }

    pub fn clear_highlights(&mut self) {
        self.highlights.clear();
    }
}
