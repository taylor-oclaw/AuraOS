extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct EmailNewsletterDigest {
    subscribers: Vec<String>,
    articles: Vec<String>,
}

impl EmailNewsletterDigest {
    pub fn new() -> Self {
        EmailNewsletterDigest {
            subscribers: Vec::new(),
            articles: Vec::new(),
        }
    }

    pub fn add_subscriber(&mut self, email: String) {
        if !self.subscribers.contains(&email) {
            self.subscribers.push(email);
        }
    }

    pub fn remove_subscriber(&mut self, email: &str) {
        self.subscribers.retain(|e| e != email);
    }

    pub fn add_article(&mut self, article: String) {
        self.articles.push(article);
    }

    pub fn get_subscribers_count(&self) -> usize {
        self.subscribers.len()
    }

    pub fn compile_digest(&self) -> Vec<String> {
        let mut digest = Vec::new();
        for subscriber in &self.subscribers {
            let mut email_content = String::from("Digest for ");
            email_content.push_str(subscriber);
            email_content.push_str(":\n");
            for article in &self.articles {
                email_content.push_str("- ");
                email_content.push_str(article);
                email_content.push('\n');
            }
            digest.push(email_content);
        }
        digest
    }
}
