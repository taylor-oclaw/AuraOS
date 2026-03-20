extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct FamilyParentalCtrl {
    user_data: Vec<String>,
    blocked_content: Vec<String>,
}

impl FamilyParentalCtrl {
    pub fn new() -> Self {
        FamilyParentalCtrl {
            user_data: Vec::new(),
            blocked_content: Vec::new(),
        }
    }

    pub fn add_user(&mut self, username: &str) {
        if !self.user_data.contains(&String::from(username)) {
            self.user_data.push(String::from(username));
        }
    }

    pub fn remove_user(&mut self, username: &str) {
        self.user_data.retain(|u| u != username);
    }

    pub fn block_content(&mut self, content: &str) {
        if !self.blocked_content.contains(&String::from(content)) {
            self.blocked_content.push(String::from(content));
        }
    }

    pub fn unblock_content(&mut self, content: &str) {
        self.blocked_content.retain(|c| c != content);
    }

    pub fn is_content_blocked(&self, content: &str) -> bool {
        self.blocked_content.contains(&String::from(content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_family_parental_ctrl() {
        let mut ctrl = FamilyParentalCtrl::new();
        assert_eq!(ctrl.user_data.len(), 0);
        assert_eq!(ctrl.blocked_content.len(), 0);

        ctrl.add_user("Alice");
        assert_eq!(ctrl.user_data.len(), 1);
        assert_eq!(ctrl.user_data[0], "Alice");

        ctrl.remove_user("Alice");
        assert_eq!(ctrl.user_data.len(), 0);

        ctrl.block_content("adult content");
        assert_eq!(ctrl.blocked_content.len(), 1);
        assert_eq!(ctrl.blocked_content[0], "adult content");

        ctrl.unblock_content("adult content");
        assert_eq!(ctrl.blocked_content.len(), 0);

        assert!(!ctrl.is_content_blocked("child content"));
        ctrl.block_content("child content");
        assert!(ctrl.is_content_blocked("child content"));
    }
}
