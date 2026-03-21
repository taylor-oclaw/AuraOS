extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct ParentalControl {
    blocked_keywords: Vec<String>,
    allowed_users: Vec<String>,
    log: Vec<String>,
}

impl ParentalControl {
    pub fn new(blocked_keywords: Vec<&str>, allowed_users: Vec<&str>) -> Self {
        let blocked_keywords = blocked_keywords.into_iter().map(String::from).collect();
        let allowed_users = allowed_users.into_iter().map(String::from).collect();
        ParentalControl {
            blocked_keywords,
            allowed_users,
            log: Vec::new(),
        }
    }

    pub fn add_blocked_keyword(&mut self, keyword: &str) {
        if !self.blocked_keywords.contains(&String::from(keyword)) {
            self.blocked_keywords.push(String::from(keyword));
        }
    }

    pub fn remove_blocked_keyword(&mut self, keyword: &str) {
        self.blocked_keywords.retain(|k| k != keyword);
    }

    pub fn add_allowed_user(&mut self, user: &str) {
        if !self.allowed_users.contains(&String::from(user)) {
            self.allowed_users.push(String::from(user));
        }
    }

    pub fn remove_allowed_user(&mut self, user: &str) {
        self.allowed_users.retain(|u| u != user);
    }

    pub fn check_content(&self, content: &str, user: &str) -> bool {
        if !self.allowed_users.contains(&String::from(user)) {
            return false;
        }
        for keyword in &self.blocked_keywords {
            if content.contains(keyword) {
                self.log.push(format!("Blocked content from {}: {}", user, content));
                return false;
            }
        }
        true
    }

    pub fn get_log(&self) -> Vec<String> {
        self.log.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parental_control() {
        let mut control = ParentalControl::new(vec!["sex"], vec!["admin"]);
        assert!(control.check_content("This is a safe message", "admin"));
        assert!(!control.check_content("This contains sex", "admin"));
        control.add_allowed_user("user");
        assert!(control.check_content("This is a safe message", "user"));
        control.remove_allowed_user("user");
        assert!(!control.check_content("This is a safe message", "user"));
        control.add_blocked_keyword("test");
        assert!(!control.check_content("This contains test", "admin"));
        let log = control.get_log();
        assert_eq!(log.len(), 1);
        assert_eq!(log[0], "Blocked content from admin: This contains sex");
    }
}
