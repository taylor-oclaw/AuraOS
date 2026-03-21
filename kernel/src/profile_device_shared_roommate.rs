extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileDeviceSharedRoommate {
    name: String,
    age: u8,
    interests: Vec<String>,
    roommates: Vec<String>,
}

impl ProfileDeviceSharedRoommate {
    pub fn new(name: &str, age: u8) -> Self {
        ProfileDeviceSharedRoommate {
            name: String::from(name),
            age,
            interests: Vec::new(),
            roommates: Vec::new(),
        }
    }

    pub fn add_interest(&mut self, interest: &str) {
        self.interests.push(String::from(interest));
    }

    pub fn remove_interest(&mut self, interest: &str) -> bool {
        if let Some(pos) = self.interests.iter().position(|i| i == interest) {
            self.interests.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn add_roommate(&mut self, roommate: &str) {
        self.roommates.push(String::from(roommate));
    }

    pub fn remove_roommate(&mut self, roommate: &str) -> bool {
        if let Some(pos) = self.roommates.iter().position(|r| r == roommate) {
            self.roommates.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn list_interests(&self) -> Vec<String> {
        self.interests.clone()
    }

    pub fn list_roommates(&self) -> Vec<String> {
        self.roommates.clone()
    }
}
