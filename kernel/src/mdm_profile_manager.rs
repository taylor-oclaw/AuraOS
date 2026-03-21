extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mdm_profile_manager_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mdm_profile_manager_exit() {
    // Cleanup logic for the module
}

pub struct ProfileManager {
    profiles: Vec<String>,
}

impl ProfileManager {
    pub fn new() -> Self {
        ProfileManager {
            profiles: Vec::new(),
        }
    }

    pub fn add_profile(&mut self, profile_name: &str) {
        if !self.profiles.contains(&String::from(profile_name)) {
            self.profiles.push(String::from(profile_name));
        }
    }

    pub fn remove_profile(&mut self, profile_name: &str) {
        self.profiles.retain(|p| p != profile_name);
    }

    pub fn list_profiles(&self) -> Vec<String> {
        self.profiles.clone()
    }

    pub fn has_profile(&self, profile_name: &str) -> bool {
        self.profiles.contains(&String::from(profile_name))
    }

    pub fn clear_profiles(&mut self) {
        self.profiles.clear();
    }
}

#[no_mangle]
pub extern "C" fn mdm_profile_manager_add_profile(profile_name: *const u8, length: usize) -> i32 {
    let profile_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(profile_name, length)) };
    let mut manager = ProfileManager::new();
    manager.add_profile(profile_str);
    0 // Return success
}

#[no_mangle]
pub extern "C" fn mdm_profile_manager_remove_profile(profile_name: *const u8, length: usize) -> i32 {
    let profile_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(profile_name, length)) };
    let mut manager = ProfileManager::new();
    manager.remove_profile(profile_str);
    0 // Return success
}

#[no_mangle]
pub extern "C" fn mdm_profile_manager_list_profiles() -> *const u8 {
    let manager = ProfileManager::new();
    let profiles = manager.list_profiles();
    let mut result = String::new();
    for profile in profiles {
        result.push_str(&profile);
        result.push(',');
    }
    if !result.is_empty() {
        result.pop(); // Remove the trailing comma
    }
    Box::leak(result.into_boxed_str()).as_ptr()
}

#[no_mangle]
pub extern "C" fn mdm_profile_manager_has_profile(profile_name: *const u8, length: usize) -> i32 {
    let profile_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(profile_name, length)) };
    let manager = ProfileManager::new();
    if manager.has_profile(profile_str) {
        1 // True
    } else {
        0 // False
    }
}

#[no_mangle]
pub extern "C" fn mdm_profile_manager_clear_profiles() -> i32 {
    let mut manager = ProfileManager::new();
    manager.clear_profiles();
    0 // Return success
}
