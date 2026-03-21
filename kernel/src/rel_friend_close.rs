extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rel_friend_close_init() {
    // Initialization logic for the module
}

pub extern "C" fn rel_friend_close_exit() {
    // Cleanup logic for the module
}

pub struct FriendManager {
    friends: Vec<String>,
}

impl FriendManager {
    pub fn new() -> Self {
        FriendManager {
            friends: Vec::new(),
        }
    }

    pub fn add_friend(&mut self, name: &str) {
        self.friends.push(String::from(name));
    }

    pub fn remove_friend(&mut self, name: &str) {
        if let Some(index) = self.friends.iter().position(|friend| friend == name) {
            self.friends.remove(index);
        }
    }

    pub fn list_friends(&self) -> Vec<String> {
        self.friends.clone()
    }

    pub fn has_friend(&self, name: &str) -> bool {
        self.friends.contains(&String::from(name))
    }

    pub fn friend_count(&self) -> usize {
        self.friends.len()
    }
}

pub extern "C" fn rel_friend_close_add_friend(manager_ptr: *mut FriendManager, name: *const u8, len: usize) {
    unsafe {
        if let Some(name_str) = core::str::from_utf8(core::slice::from_raw_parts(name, len)).ok() {
            (*manager_ptr).add_friend(name_str);
        }
    }
}

pub extern "C" fn rel_friend_close_remove_friend(manager_ptr: *mut FriendManager, name: *const u8, len: usize) {
    unsafe {
        if let Some(name_str) = core::str::from_utf8(core::slice::from_raw_parts(name, len)).ok() {
            (*manager_ptr).remove_friend(name_str);
        }
    }
}

pub extern "C" fn rel_friend_close_list_friends(manager_ptr: *const FriendManager, friends_ptr: *mut *const u8, lengths_ptr: *mut usize) -> usize {
    unsafe {
        let manager = &*manager_ptr;
        let friends = manager.list_friends();
        let count = friends.len();

        for i in 0..count {
            (*friends_ptr).add(friends[i].as_ptr());
            (*lengths_ptr).add(friends[i].len());
        }

        count
    }
}

pub extern "C" fn rel_friend_close_has_friend(manager_ptr: *const FriendManager, name: *const u8, len: usize) -> bool {
    unsafe {
        if let Some(name_str) = core::str::from_utf8(core::slice::from_raw_parts(name, len)).ok() {
            (*manager_ptr).has_friend(name_str)
        } else {
            false
        }
    }
}

pub extern "C" fn rel_friend_close_friend_count(manager_ptr: *const FriendManager) -> usize {
    unsafe {
        (*manager_ptr).friend_count()
    }
}
