extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct APFSVolume {
    name: String,
    size: u64,
    files: Vec<String>,
}

impl APFSVolume {
    pub fn new(name: &str, size: u64) -> Self {
        APFSVolume {
            name: String::from(name),
            size,
            files: Vec::new(),
        }
    }

    pub fn add_file(&mut self, file_name: &str) {
        self.files.push(String::from(file_name));
    }

    pub fn remove_file(&mut self, file_name: &str) -> bool {
        if let Some(index) = self.files.iter().position(|f| f == file_name) {
            self.files.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_files(&self) -> Vec<String> {
        self.files.clone()
    }

    pub fn get_volume_size(&self) -> u64 {
        self.size
    }

    pub fn rename_volume(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }
}

pub extern "C" fn create_apfs_volume(name: *const u8, size: u64) -> *mut APFSVolume {
    let name_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(name, 0)) };
    Box::into_raw(Box::new(APFSVolume::new(name_str, size)))
}

pub extern "C" fn add_file_to_volume(volume: *mut APFSVolume, file_name: *const u8) {
    let file_name_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(file_name, 0)) };
    unsafe { (*volume).add_file(file_name_str); }
}

pub extern "C" fn remove_file_from_volume(volume: *mut APFSVolume, file_name: *const u8) -> bool {
    let file_name_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(file_name, 0)) };
    unsafe { (*volume).remove_file(file_name_str) }
}

pub extern "C" fn list_files_in_volume(volume: *const APFSVolume) -> Vec<String> {
    unsafe { (*volume).list_files() }
}

pub extern "C" fn get_volume_size(volume: *const APFSVolume) -> u64 {
    unsafe { (*volume).get_volume_size() }
}

pub extern "C" fn rename_apfs_volume(volume: *mut APFSVolume, new_name: *const u8) {
    let new_name_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(new_name, 0)) };
    unsafe { (*volume).rename_volume(new_name_str); }
}
