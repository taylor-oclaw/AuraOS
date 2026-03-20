extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AgentCodeReviewer {
    codebase: Vec<String>,
    review_comments: Vec<String>,
}

impl AgentCodeReviewer {
    pub fn new() -> Self {
        AgentCodeReviewer {
            codebase: Vec::new(),
            review_comments: Vec::new(),
        }
    }

    pub fn add_code(&mut self, code: String) {
        self.codebase.push(code);
    }

    pub fn get_code(&self, index: usize) -> Option<&String> {
        self.codebase.get(index)
    }

    pub fn review_code(&mut self) {
        for code in &self.codebase {
            let comment = self.analyze_code(code);
            self.review_comments.push(comment);
        }
    }

    fn analyze_code(&self, code: &str) -> String {
        // Simple analysis logic
        if code.contains("TODO") {
            String::from("info")
        } else if code.contains("panic!") {
            String::from("info")
        } else {
            String::from("info")
        }
    }

    pub fn get_review_comments(&self) -> &Vec<String> {
        &self.review_comments
    }
}
