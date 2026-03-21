extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_accent_chinese_cantonese_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn lang_accent_chinese_cantonese_exit() {
    // Cleanup logic for the module
}

pub struct CantoneseDictionary {
    entries: Vec<(String, String)>,
}

impl CantoneseDictionary {
    pub fn new() -> Self {
        CantoneseDictionary {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, mandarin: &str, cantonese: &str) {
        let mandarin_str = String::from(mandarin);
        let cantonese_str = String::from(cantonese);
        self.entries.push((mandarin_str, cantonese_str));
    }

    pub fn get_cantonese(&self, mandarin: &str) -> Option<&String> {
        for (m, c) in &self.entries {
            if m == mandarin {
                return Some(c);
            }
        }
        None
    }

    pub fn remove_entry(&mut self, mandarin: &str) {
        self.entries.retain(|(m, _)| m != mandarin);
    }

    pub fn list_entries(&self) -> Vec<(String, String)> {
        self.entries.clone()
    }
}

#[no_mangle]
pub extern "C" fn lang_accent_chinese_cantonese_add_entry(dict: *mut CantoneseDictionary, mandarin: *const u8, cantonese: *const u8) {
    unsafe {
        if let Some(mand_str) = core::str::from_utf8(core::slice::from_raw_parts(mandarin, libc::strlen(mandarin as *const i8))) {
            if let Some(can_str) = core::str::from_utf8(core::slice::from_raw_parts(cantonese, libc::strlen(cantonese as *const i8))) {
                (*dict).add_entry(mand_str, can_str);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn lang_accent_chinese_cantonese_get_cantonese(dict: *const CantoneseDictionary, mandarin: *const u8) -> *const u8 {
    unsafe {
        if let Some(mand_str) = core::str::from_utf8(core::slice::from_raw_parts(mandarin, libc::strlen(mandarin as *const i8))) {
            if let Some(cantonese) = (*dict).get_cantonese(mand_str) {
                return cantonese.as_ptr();
            }
        }
    }
    core::ptr::null()
}

#[no_mangle]
pub extern "C" fn lang_accent_chinese_cantonese_remove_entry(dict: *mut CantoneseDictionary, mandarin: *const u8) {
    unsafe {
        if let Some(mand_str) = core::str::from_utf8(core::slice::from_raw_parts(mandarin, libc::strlen(mandarin as *const i8))) {
            (*dict).remove_entry(mand_str);
        }
    }
}

#[no_mangle]
pub extern "C" fn lang_accent_chinese_cantonese_list_entries(dict: *const CantoneseDictionary) -> Vec<(String, String)> {
    unsafe { (*dict).list_entries() }
}
