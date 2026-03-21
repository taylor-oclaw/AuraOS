extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeetingHighlightReel {
    highlights: Vec<String>,
}

impl MeetingHighlightReel {
    pub fn new() -> Self {
        MeetingHighlightReel {
            highlights: Vec::new(),
        }
    }

    pub fn add_highlight(&mut self, highlight: String) {
        self.highlights.push(highlight);
    }

    pub fn get_highlights(&self) -> &Vec<String> {
        &self.highlights
    }

    pub fn remove_highlight(&mut self, index: usize) -> Option<String> {
        if index < self.highlights.len() {
            Some(self.highlights.remove(index))
        } else {
            None
        }
    }

    pub fn clear_highlights(&mut self) {
        self.highlights.clear();
    }

    pub fn has_highlights(&self) -> bool {
        !self.highlights.is_empty()
    }
}
