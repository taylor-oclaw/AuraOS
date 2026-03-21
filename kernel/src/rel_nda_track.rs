extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rel_nda_track_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn rel_nda_track_exit() {
    // Cleanup logic for the module
}

pub struct NdaTracker {
    entries: Vec<String>,
}

impl NdaTracker {
    pub fn new() -> Self {
        NdaTracker {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: String) {
        self.entries.push(entry);
    }

    pub fn remove_entry(&mut self, index: usize) -> Option<String> {
        if index < self.entries.len() {
            Some(self.entries.remove(index))
        } else {
            None
        }
    }

    pub fn get_entries(&self) -> &[String] {
        &self.entries
    }

    pub fn find_entry(&self, entry: &str) -> Option<usize> {
        self.entries.iter().position(|e| e == entry)
    }

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }
}

#[no_mangle]
pub extern "C" fn rel_nda_track_add_entry(tracker_ptr: *mut NdaTracker, entry: *const u8, len: usize) -> i32 {
    if tracker_ptr.is_null() || entry.is_null() || len == 0 {
        return -1;
    }

    unsafe {
        let tracker = &mut *tracker_ptr;
        let entry_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(entry, len));
        tracker.add_entry(String::from(entry_str));
    }
    0
}

#[no_mangle]
pub extern "C" fn rel_nda_track_remove_entry(tracker_ptr: *mut NdaTracker, index: usize) -> i32 {
    if tracker_ptr.is_null() {
        return -1;
    }

    unsafe {
        let tracker = &mut *tracker_ptr;
        if tracker.remove_entry(index).is_some() {
            0
        } else {
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn rel_nda_track_get_entries(tracker_ptr: *const NdaTracker, entries_ptr: *mut *const u8, lengths_ptr: *mut usize) -> i32 {
    if tracker_ptr.is_null() || entries_ptr.is_null() || lengths_ptr.is_null() {
        return -1;
    }

    unsafe {
        let tracker = &*tracker_ptr;
        let entries = tracker.get_entries();
        let num_entries = entries.len();

        *entries_ptr = core::ptr::null_mut();
        *lengths_ptr = 0;

        if num_entries > 0 {
            let mut entry_ptrs: Vec<*const u8> = Vec::with_capacity(num_entries);
            let mut lengths: Vec<usize> = Vec::with_capacity(num_entries);

            for entry in entries {
                entry_ptrs.push(entry.as_bytes().as_ptr());
                lengths.push(entry.len());
            }

            *entries_ptr = entry_ptrs.as_ptr();
            *lengths_ptr = num_entries;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn rel_nda_track_find_entry(tracker_ptr: *const NdaTracker, entry: *const u8, len: usize) -> i32 {
    if tracker_ptr.is_null() || entry.is_null() || len == 0 {
        return -1;
    }

    unsafe {
        let tracker = &*tracker_ptr;
        let entry_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(entry, len));
        match tracker.find_entry(entry_str) {
            Some(index) => index as i32,
            None => -1,
        }
    }
}

#[no_mangle]
pub extern "C" fn rel_nda_track_clear_entries(tracker_ptr: *mut NdaTracker) -> i32 {
    if tracker_ptr.is_null() {
        return -1;
    }

    unsafe {
        let tracker = &mut *tracker_ptr;
        tracker.clear_entries();
    }
    0
}
