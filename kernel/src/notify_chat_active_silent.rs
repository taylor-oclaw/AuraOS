extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct NotifyChatActiveSilent {
    active_users: Vec<String>,
    silent_mode: bool,
}

impl NotifyChatActiveSilent {
    pub fn new() -> Self {
        NotifyChatActiveSilent {
            active_users: Vec::new(),
            silent_mode: false,
        }
    }

    pub fn add_user(&mut self, user: String) {
        if !self.active_users.contains(&user) {
            self.active_users.push(user);
        }
    }

    pub fn remove_user(&mut self, user: &str) {
        self.active_users.retain(|u| u != user);
    }

    pub fn toggle_silent_mode(&mut self) {
        self.silent_mode = !self.silent_mode;
    }

    pub fn is_user_active(&self, user: &str) -> bool {
        self.active_users.contains(&String::from(user))
    }

    pub fn get_active_users(&self) -> Vec<String> {
        self.active_users.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notify_chat_active_silent() {
        let mut notifier = NotifyChatActiveSilent::new();
        assert_eq!(notifier.get_active_users().len(), 0);
        assert!(!notifier.is_user_active("user1"));

        notifier.add_user(String::from("user1"));
        assert_eq!(notifier.get_active_users().len(), 1);
        assert!(notifier.is_user_active("user1"));

        notifier.remove_user("user1");
        assert_eq!(notifier.get_active_users().len(), 0);
        assert!(!notifier.is_user_active("user1"));

        notifier.toggle_silent_mode();
        assert!(notifier.silent_mode);

        notifier.toggle_silent_mode();
        assert!(!notifier.silent_mode);
    }
}
