extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DocumentContractAnalyze {
    document_content: String,
    keywords: Vec<String>,
}

impl DocumentContractAnalyze {
    pub fn new(document_content: &str) -> Self {
        DocumentContractAnalyze {
            document_content: String::from(document_content),
            keywords: Vec::new(),
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

    pub fn contains_keyword(&self, keyword: &str) -> bool {
        self.keywords.contains(&String::from(keyword))
    }

    pub fn analyze_document(&self) -> Vec<String> {
        let mut results = Vec::new();
        for keyword in &self.keywords {
            if self.document_content.contains(keyword) {
                results.push(String::from(keyword));
            }
        }
        results
    }

    pub fn get_keywords_count(&self) -> usize {
        self.keywords.len()
    }
}
