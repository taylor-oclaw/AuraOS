extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn native_skill_ffi_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn native_skill_ffi_exit() {
    // Cleanup logic for the module
}

pub struct NativeSkillFFI {
    skills: Vec<String>,
}

impl NativeSkillFFI {
    pub fn new() -> Self {
        NativeSkillFFI {
            skills: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill: String) {
        self.skills.push(skill);
    }

    pub fn remove_skill(&mut self, index: usize) -> Option<String> {
        if index < self.skills.len() {
            Some(self.skills.remove(index))
        } else {
            None
        }
    }

    pub fn get_skill_count(&self) -> usize {
        self.skills.len()
    }

    pub fn list_skills(&self) -> Vec<&str> {
        self.skills.iter().map(|s| s.as_str()).collect()
    }

    pub fn has_skill(&self, skill: &str) -> bool {
        self.skills.contains(&String::from(skill))
    }
}
