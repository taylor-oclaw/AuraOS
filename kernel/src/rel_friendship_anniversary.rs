extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct FriendshipAnniversary {
    name: String,
    anniversary_date: String,
    friends: Vec<String>,
}

impl FriendshipAnniversary {
    pub fn new(name: &str, anniversary_date: &str) -> Self {
        FriendshipAnniversary {
            name: String::from(name),
            anniversary_date: String::from(anniversary_date),
            friends: Vec::new(),
        }
    }

    pub fn add_friend(&mut self, friend_name: &str) {
        self.friends.push(String::from(friend_name));
    }

    pub fn remove_friend(&mut self, friend_name: &str) -> bool {
        if let Some(index) = self.friends.iter().position(|f| f == friend_name) {
            self.friends.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_friends(&self) -> Vec<String> {
        self.friends.clone()
    }

    pub fn get_anniversary_date(&self) -> String {
        self.anniversary_date.clone()
    }

    pub fn set_anniversary_date(&mut self, new_date: &str) {
        self.anniversary_date = String::from(new_date);
    }
}
