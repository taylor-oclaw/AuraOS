extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rel_video_pref_detect_init() {
    // Initialization logic for the module
}

pub extern "C" fn rel_video_pref_detect_exit() {
    // Cleanup logic for the module
}

pub struct VideoPreferenceDetector {
    preferences: Vec<String>,
}

impl VideoPreferenceDetector {
    pub fn new() -> Self {
        VideoPreferenceDetector {
            preferences: Vec::new(),
        }
    }

    pub fn add_preference(&mut self, preference: String) {
        self.preferences.push(preference);
    }

    pub fn remove_preference(&mut self, preference: &str) -> bool {
        if let Some(index) = self.preferences.iter().position(|p| p == preference) {
            self.preferences.remove(index);
            true
        } else {
            false
        }
    }

    pub fn has_preference(&self, preference: &str) -> bool {
        self.preferences.contains(&String::from(preference))
    }

    pub fn list_preferences(&self) -> Vec<String> {
        self.preferences.clone()
    }

    pub fn clear_preferences(&mut self) {
        self.preferences.clear();
    }
}

pub extern "C" fn rel_video_pref_detect_add_preference(detector: *mut VideoPreferenceDetector, preference: *const u8, len: usize) -> bool {
    if detector.is_null() || preference.is_null() {
        return false;
    }

    unsafe {
        let slice = core::slice::from_raw_parts(preference, len);
        if let Ok(pref_str) = core::str::from_utf8(slice) {
            (*detector).add_preference(String::from(pref_str));
            true
        } else {
            false
        }
    }
}

pub extern "C" fn rel_video_pref_detect_remove_preference(detector: *mut VideoPreferenceDetector, preference: *const u8, len: usize) -> bool {
    if detector.is_null() || preference.is_null() {
        return false;
    }

    unsafe {
        let slice = core::slice::from_raw_parts(preference, len);
        if let Ok(pref_str) = core::str::from_utf8(slice) {
            (*detector).remove_preference(pref_str)
        } else {
            false
        }
    }
}

pub extern "C" fn rel_video_pref_detect_has_preference(detector: *const VideoPreferenceDetector, preference: *const u8, len: usize) -> bool {
    if detector.is_null() || preference.is_null() {
        return false;
    }

    unsafe {
        let slice = core::slice::from_raw_parts(preference, len);
        if let Ok(pref_str) = core::str::from_utf8(slice) {
            (*detector).has_preference(pref_str)
        } else {
            false
        }
    }
}

pub extern "C" fn rel_video_pref_detect_list_preferences(detector: *const VideoPreferenceDetector, buffer: *mut u8, buffer_len: usize) -> isize {
    if detector.is_null() || buffer.is_null() {
        return -1;
    }

    unsafe {
        let preferences = (*detector).list_preferences();
        let total_len: usize = preferences.iter().map(|p| p.len() + 1).sum(); // +1 for null terminator

        if total_len > buffer_len {
            return -2; // Not enough space in the buffer
        }

        let mut current_offset = 0;
        for pref in preferences {
            let bytes_written = core::ptr::copy_nonoverlapping(pref.as_ptr(), buffer.offset(current_offset as isize), pref.len());
            current_offset += bytes_written + 1; // Move past the string and null terminator
            *buffer.offset((current_offset - 1) as isize) = 0; // Null terminate each string
        }

        total_len as isize
    }
}

pub extern "C" fn rel_video_pref_detect_clear_preferences(detector: *mut VideoPreferenceDetector) {
    if !detector.is_null() {
        unsafe {
            (*detector).clear_preferences();
        }
    }
}
