extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct NotificationSpamFilter {
    blocked_keywords: Vec<String>,
    allowed_senders: Vec<String>,
}

impl NotificationSpamFilter {
    pub fn new() -> Self {
        NotificationSpamFilter {
            blocked_keywords: Vec::new(),
            allowed_senders: Vec::new(),
        }
    }

    pub fn add_blocked_keyword(&mut self, keyword: String) {
        if !self.blocked_keywords.contains(&keyword) {
            self.blocked_keywords.push(keyword);
        }
    }

    pub fn remove_blocked_keyword(&mut self, keyword: &str) -> bool {
        let index = self.blocked_keywords.iter().position(|k| k == keyword);
        if let Some(i) = index {
            self.blocked_keywords.remove(i);
            true
        } else {
            false
        }
    }

    pub fn add_allowed_sender(&mut self, sender: String) {
        if !self.allowed_senders.contains(&sender) {
            self.allowed_senders.push(sender);
        }
    }

    pub fn remove_allowed_sender(&mut self, sender: &str) -> bool {
        let index = self.allowed_senders.iter().position(|s| s == sender);
        if let Some(i) = index {
            self.allowed_senders.remove(i);
            true
        } else {
            false
        }
    }

    pub fn is_spam(&self, notification: &str, sender: &str) -> bool {
        if self.allowed_senders.contains(&sender.to_string()) {
            return false;
        }

        for keyword in &self.blocked_keywords {
            if notification.contains(keyword) {
                return true;
            }
        }

        false
    }
}
