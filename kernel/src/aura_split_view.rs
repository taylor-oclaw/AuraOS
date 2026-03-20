extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraSplitView {
    title: String,
    content_left: Vec<String>,
    content_right: Vec<String>,
    selected_index: usize,
}

impl AuraSplitView {
    pub fn new(title: &str) -> Self {
        AuraSplitView {
            title: String::from(title),
            content_left: Vec::new(),
            content_right: Vec::new(),
            selected_index: 0,
        }
    }

    pub fn add_to_left(&mut self, item: &str) {
        self.content_left.push(String::from(item));
    }

    pub fn add_to_right(&mut self, item: &str) {
        self.content_right.push(String::from(item));
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn select_next(&mut self) {
        if self.selected_index < self.content_left.len() + self.content_right.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn select_previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn get_selected_item(&self) -> Option<&str> {
        if self.selected_index < self.content_left.len() {
            Some(&self.content_left[self.selected_index])
        } else if self.selected_index < self.content_left.len() + self.content_right.len() {
            Some(&self.content_right[self.selected_index - self.content_left.len()])
        } else {
            None
        }
    }
}
