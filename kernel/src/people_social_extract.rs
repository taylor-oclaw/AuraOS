extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn people_social_extract_init() {
    // Initialization code for the module
}

#[no_mangle]
pub extern "C" fn people_social_extract_exit() {
    // Cleanup code for the module
}

pub struct PeopleSocialExtractor {
    data: Vec<String>,
}

impl PeopleSocialExtractor {
    pub fn new() -> Self {
        PeopleSocialExtractor {
            data: Vec::new(),
        }
    }

    pub fn add_person(&mut self, name: &str) {
        self.data.push(name.to_string());
    }

    pub fn remove_person(&mut self, name: &str) {
        if let Some(index) = self.data.iter().position(|x| x == name) {
            self.data.remove(index);
        }
    }

    pub fn get_people_count(&self) -> usize {
        self.data.len()
    }

    pub fn list_people(&self) -> Vec<String> {
        self.data.clone()
    }

    pub fn find_person(&self, name: &str) -> bool {
        self.data.contains(&name.to_string())
    }
}

#[no_mangle]
pub extern "C" fn people_social_extractor_new() -> *mut PeopleSocialExtractor {
    Box::into_raw(Box::new(PeopleSocialExtractor::new()))
}

#[no_mangle]
pub extern "C" fn people_social_extractor_add_person(extractor: *mut PeopleSocialExtractor, name: *const u8, len: usize) {
    unsafe {
        let slice = core::slice::from_raw_parts(name, len);
        if let Ok(s) = core::str::from_utf8(slice) {
            (*extractor).add_person(s);
        }
    }
}

#[no_mangle]
pub extern "C" fn people_social_extractor_remove_person(extractor: *mut PeopleSocialExtractor, name: *const u8, len: usize) {
    unsafe {
        let slice = core::slice::from_raw_parts(name, len);
        if let Ok(s) = core::str::from_utf8(slice) {
            (*extractor).remove_person(s);
        }
    }
}

#[no_mangle]
pub extern "C" fn people_social_extractor_get_people_count(extractor: *const PeopleSocialExtractor) -> usize {
    unsafe { (*extractor).get_people_count() }
}

#[no_mangle]
pub extern "C" fn people_social_extractor_list_people(extractor: *const PeopleSocialExtractor, count_ptr: *mut usize) -> *mut *const u8 {
    let count = unsafe { (*count_ptr) };
    let mut names: Vec<*const u8> = Vec::with_capacity(count);
    for name in unsafe { &(*extractor).list_people() } {
        names.push(name.as_ptr());
    }
    unsafe { *count_ptr = names.len(); }
    Box::into_raw(Box::new(names)) as *mut *const u8
}

#[no_mangle]
pub extern "C" fn people_social_extractor_find_person(extractor: *const PeopleSocialExtractor, name: *const u8, len: usize) -> bool {
    unsafe {
        let slice = core::slice::from_raw_parts(name, len);
        if let Ok(s) = core::str::from_utf8(slice) {
            (*extractor).find_person(s)
        } else {
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn people_social_extractor_free(extractor: *mut PeopleSocialExtractor) {
    unsafe { drop(Box::from_raw(extractor)); }
}
