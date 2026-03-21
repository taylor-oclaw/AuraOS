extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod buddy_system; // Assuming a buddy system memory allocator is implemented

pub struct RelFriendCasual {
    friends: Vec<String>,
}

impl RelFriendCasual {
    pub fn new() -> Self {
        RelFriendCasual {
            friends: Vec::new(),
        }
    }

    pub fn add_friend(&mut self, name: &str) {
        let friend_name = String::from(name);
        if !self.friends.contains(&friend_name) {
            self.friends.push(friend_name);
        }
    }

    pub fn remove_friend(&mut self, name: &str) -> bool {
        let friend_name = String::from(name);
        if let Some(index) = self.friends.iter().position(|x| *x == friend_name) {
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
        let friend_name = String::from(name);
        self.friends.contains(&friend_name)
    }

    pub fn count_friends(&self) -> usize {
        self.friends.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_friend() {
        let mut rel = RelFriendCasual::new();
        rel.add_friend("Alice");
        assert_eq!(rel.count_friends(), 1);
    }

    #[test]
    fn test_remove_friend() {
        let mut rel = RelFriendCasual::new();
        rel.add_friend("Bob");
        assert!(rel.remove_friend("Bob"));
        assert_eq!(rel.count_friends(), 0);
    }

    #[test]
    fn test_list_friends() {
        let mut rel = RelFriendCasual::new();
        rel.add_friend("Charlie");
        rel.add_friend("David");
        let friends = rel.list_friends();
        assert_eq!(friends.len(), 2);
        assert!(friends.contains(&String::from("Charlie")));
        assert!(friends.contains(&String::from("David")));
    }

    #[test]
    fn test_is_friend() {
        let mut rel = RelFriendCasual::new();
        rel.add_friend("Eve");
        assert!(rel.is_friend("Eve"));
        assert!(!rel.is_friend("Frank"));
    }

    #[test]
    fn test_count_friends() {
        let mut rel = RelFriendCasual::new();
        rel.add_friend("Grace");
        rel.add_friend("Heidi");
        assert_eq!(rel.count_friends(), 2);
    }
}
