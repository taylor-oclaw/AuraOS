extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct DocumentCommentSmart {
    comments: Vec<String>,
}

impl DocumentCommentSmart {
    pub fn new() -> Self {
        DocumentCommentSmart {
            comments: Vec::new(),
        }
    }

    pub fn add_comment(&mut self, comment: String) {
        self.comments.push(comment);
    }

    pub fn get_comments(&self) -> &Vec<String> {
        &self.comments
    }

    pub fn remove_comment(&mut self, index: usize) -> Option<String> {
        if index < self.comments.len() {
            Some(self.comments.remove(index))
        } else {
            None
        }
    }

    pub fn find_comments_with_keyword(&self, keyword: &str) -> Vec<&String> {
        self.comments.iter().filter(|comment| comment.contains(keyword)).collect()
    }

    pub fn clear_comments(&mut self) {
        self.comments.clear();
    }
}
