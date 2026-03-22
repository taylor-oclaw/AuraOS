#![no_std]
#![feature(allocator_api)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::alloc::{AllocError, Allocator, Global};
use core::ptr::NonNull;

struct MdmProfile {
    name: String,
    settings: Vec<String>,
}

impl MdmProfile {
    pub fn new(name: String) -> Self {
        MdmProfile {
            name,
            settings: Vec::new(),
        }
    }

    pub fn add_setting(&mut self, setting: String) {
        self.settings.push(setting);
    }

    pub fn get_settings(&self) -> &Vec<String> {
        &self.settings
    }

    pub fn remove_setting(&mut self, index: usize) -> Option<String> {
        if index < self.settings.len() {
            Some(self.settings.remove(index))
        } else {
            None
        }
    }

    pub fn clear_settings(&mut self) {
        self.settings.clear();
    }
}

struct MdmProfileManager {
    profiles: Vec<MdmProfile>,
}

impl MdmProfileManager {
    pub fn new() -> Self {
        MdmProfileManager {
            profiles: Vec::new(),
        }
    }

    pub fn add_profile(&mut self, profile: MdmProfile) {
        self.profiles.push(profile);
    }

    pub fn get_profiles(&self) -> &Vec<MdmProfile> {
        &self.profiles
    }

    pub fn remove_profile(&mut self, index: usize) -> Option<MdmProfile> {
        if index < self.profiles.len() {
            Some(self.profiles.remove(index))
        } else {
            None
        }
    }

    pub fn clear_profiles(&mut self) {
        self.profiles.clear();
    }
}

struct MdmProfileManagerAllocator;

unsafe impl Allocator for MdmProfileManagerAllocator {
    fn allocate(&self, layout: core::alloc::Layout) -> Result<NonNull<u8>, AllocError> {
        Global.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: core::alloc::Layout) {
        Global.deallocate(ptr, layout);
    }
}

static ALLOCATOR: MdmProfileManagerAllocator = MdmProfileManagerAllocator;

#[global_allocator]
static GLOBAL_ALLOC: &'static dyn Allocator = &ALLOCATOR;