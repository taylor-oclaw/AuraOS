extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MarketplaceModeration {
    banned_users: Vec<String>,
    banned_keywords: Vec<String>,
}

impl MarketplaceModeration {
    pub fn new() -> Self {
        MarketplaceModeration {
            banned_users: Vec::new(),
            banned_keywords: Vec::new(),
        }
    }

    pub fn add_banned_user(&mut self, user_id: String) {
        self.banned_users.push(user_id);
    }

    pub fn remove_banned_user(&mut self, user_id: &String) -> bool {
        if let Some(pos) = self.banned_users.iter().position(|x| x == user_id) {
            self.banned_users.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn add_banned_keyword(&mut self, keyword: String) {
        self.banned_keywords.push(keyword);
    }

    pub fn remove_banned_keyword(&mut self, keyword: &String) -> bool {
        if let Some(pos) = self.banned_keywords.iter().position(|x| x == keyword) {
            self.banned_keywords.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn check_user(&self, user_id: &String) -> bool {
        self.banned_users.contains(user_id)
    }

    pub fn check_keyword(&self, message: &String) -> bool {
        for keyword in &self.banned_keywords {
            if message.contains(keyword) {
                return true;
            }
        }
        false
    }
}