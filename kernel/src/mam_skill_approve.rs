#![no_std]
#![feature(allocator_api)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

struct MamSkillApprove {
    skills: Vec<String>,
}

impl MamSkillApprove {
    pub fn new() -> Self {
        MamSkillApprove { skills: Vec::new() }
    }

    pub fn add_skill(&mut self, skill: String) {
        self.skills.push(skill);
    }

    pub fn remove_skill(&mut self, skill: &str) -> bool {
        if let Some(index) = self.skills.iter().position(|s| s == skill) {
            self.skills.remove(index);
            true
        } else {
            false
        }
    }

    pub fn has_skill(&self, skill: &str) -> bool {
        self.skills.contains(skill)
    }

    pub fn list_skills(&self) -> Vec<&String> {
        self.skills.iter().collect()
    }
}

#[no_mangle]
pub extern "C" fn mam_skill_approve_init() -> *mut MamSkillApprove {
    Box::into_raw(Box::new(MamSkillApprove::new()))
}

#[no_mangle]
pub extern "C" fn mam_skill_approve_add_skill(ptr: *mut MamSkillApprove, skill: *const u8) {
    let skill_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(skill, 1)) };
    unsafe { &mut *ptr }.add_skill(String::from(skill_str));
}

#[no_mangle]
pub extern "C" fn mam_skill_approve_remove_skill(ptr: *mut MamSkillApprove, skill: *const u8) -> bool {
    let skill_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(skill, 1)) };
    unsafe { &mut *ptr }.remove_skill(skill_str)
}

#[no_mangle]
pub extern "C" fn mam_skill_approve_has_skill(ptr: *const MamSkillApprove, skill: *const u8) -> bool {
    let skill_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(skill, 1)) };
    unsafe { &*ptr }.has_skill(skill_str)
}

#[no_mangle]
pub extern "C" fn mam_skill_approve_list_skills(ptr: *const MamSkillApprove) -> *mut Vec<&'static str> {
    let skills = unsafe { &*ptr }.list_skills();
    Box::into_raw(Box::new(skills.iter().map(|s| s.as_str()).collect()))
}