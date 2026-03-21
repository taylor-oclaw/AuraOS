extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AIToxicityFilter {
    keywords: Vec<String>,
}

impl AIToxicityFilter {
    pub fn new(keywords: Vec<&str>) -> Self {
        let mut keyword_vec = Vec::new();
        for word in keywords {
            keyword_vec.push(String::from(word));
        }
        AIToxicityFilter {
            keywords: keyword_vec,
        }
    }

    pub fn add_keyword(&mut self, keyword: &str) {
        self.keywords.push(String::from(keyword));
    }

    pub fn remove_keyword(&mut self, keyword: &str) -> bool {
        if let Some(index) = self.keywords.iter().position(|k| k == keyword) {
            self.keywords.remove(index);
            true
        } else {
            false
        }
    }

    pub fn contains_toxicity(&self, text: &str) -> bool {
        for keyword in &self.keywords {
            if text.contains(keyword) {
                return true;
            }
        }
        false
    }

    pub fn list_keywords(&self) -> Vec<String> {
        self.keywords.clone()
    }

    pub fn clear_keywords(&mut self) {
        self.keywords.clear();
    }
}
