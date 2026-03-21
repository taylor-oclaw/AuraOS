extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rel_mute_manage_init() {
    // Initialization logic for the module
}

pub extern "C" fn rel_mute_manage_exit() {
    // Cleanup logic for the module
}

pub struct MuteManager {
    mutes: Vec<String>,
}

impl MuteManager {
    pub fn new() -> Self {
        MuteManager { mutes: Vec::new() }
    }

    pub fn add_mute(&mut self, mute: String) {
        if !self.mutes.contains(&mute) {
            self.mutes.push(mute);
        }
    }

    pub fn remove_mute(&mut self, mute: &str) {
        self.mutes.retain(|m| m != mute);
    }

    pub fn is_muted(&self, mute: &str) -> bool {
        self.mutes.contains(&String::from(mute))
    }

    pub fn list_mutes(&self) -> Vec<String> {
        self.mutes.clone()
    }

    pub fn clear_mutes(&mut self) {
        self.mutes.clear();
    }
}

pub extern "C" fn rel_mute_manage_add_mute(mute: *const u8, len: usize) -> i32 {
    let mute_str = unsafe { core::slice::from_raw_parts(mute, len) };
    if let Ok(s) = String::from_utf8(mute_str.to_vec()) {
        let manager = MuteManager::new();
        manager.add_mute(s);
        0
    } else {
        -1
    }
}

pub extern "C" fn rel_mute_manage_remove_mute(mute: *const u8, len: usize) -> i32 {
    let mute_str = unsafe { core::slice::from_raw_parts(mute, len) };
    if let Ok(s) = String::from_utf8(mute_str.to_vec()) {
        let manager = MuteManager::new();
        manager.remove_mute(&s);
        0
    } else {
        -1
    }
}

pub extern "C" fn rel_mute_manage_is_muted(mute: *const u8, len: usize) -> i32 {
    let mute_str = unsafe { core::slice::from_raw_parts(mute, len) };
    if let Ok(s) = String::from_utf8(mute_str.to_vec()) {
        let manager = MuteManager::new();
        if manager.is_muted(&s) {
            1
        } else {
            0
        }
    } else {
        -1
    }
}

pub extern "C" fn rel_mute_manage_list_mutes() -> *const u8 {
    let manager = MuteManager::new();
    let mutes = manager.list_mutes();
    // This is a simplified example, in practice you would need to handle memory allocation and deallocation
    // for the returned data.
    core::ptr::null()
}

pub extern "C" fn rel_mute_manage_clear_mutes() {
    let mut manager = MuteManager::new();
    manager.clear_mutes();
}
