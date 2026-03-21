extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn tone_close_friend_mode_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn tone_close_friend_mode_exit() {
    // Cleanup logic for the module
}

pub struct ToneCloseFriendMode {
    friends: Vec<String>,
    max_friends: usize,
}

impl ToneCloseFriendMode {
    pub fn new(max_friends: usize) -> Self {
        ToneCloseFriendMode {
            friends: Vec::new(),
            max_friends,
        }
    }

    pub fn add_friend(&mut self, name: &str) -> bool {
        if self.friends.len() < self.max_friends {
            self.friends.push(String::from(name));
            true
        } else {
            false
        }
    }

    pub fn remove_friend(&mut self, name: &str) -> bool {
        let pos = self.friends.iter().position(|x| x == name);
        if let Some(index) = pos {
            self.friends.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_friends(&self) -> Vec<String> {
        self.friends.clone()
    }

    pub fn is_friend(&self, name: &str) -> bool {
        self.friends.contains(&String::from(name))
    }

    pub fn friend_count(&self) -> usize {
        self.friends.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_remove_friends() {
        let mut mode = ToneCloseFriendMode::new(2);
        assert!(mode.add_friend("Alice"));
        assert!(mode.add_friend("Bob"));
        assert!(!mode.add_friend("Charlie")); // Should not add, max reached

        assert!(mode.is_friend("Alice"));
        assert!(mode.is_friend("Bob"));
        assert!(!mode.is_friend("Charlie"));

        assert!(mode.remove_friend("Alice"));
        assert!(!mode.remove_friend("Alice")); // Already removed
    }

    #[test]
    fn test_list_friends() {
        let mut mode = ToneCloseFriendMode::new(2);
        mode.add_friend("Alice");
        mode.add_friend("Bob");

        let friends = mode.list_friends();
        assert_eq!(friends, vec![String::from("Alice"), String::from("Bob")]);
    }

    #[test]
    fn test_friend_count() {
        let mut mode = ToneCloseFriendMode::new(2);
        assert_eq!(mode.friend_count(), 0);

        mode.add_friend("Alice");
        assert_eq!(mode.friend_count(), 1);

        mode.add_friend("Bob");
        assert_eq!(mode.friend_count(), 2);

        mode.remove_friend("Alice");
        assert_eq!(mode.friend_count(), 1);
    }
}
