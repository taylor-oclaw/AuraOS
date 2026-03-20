extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn crucible_ffi_init() {
    // Initialization code for the module
}

pub extern "C" fn crucible_ffi_exit() {
    // Cleanup code for the module
}

pub struct CrucibleFFI {
    data: Vec<u8>,
    name: String,
}

impl CrucibleFFI {
    pub fn new(name: &str) -> Self {
        CrucibleFFI {
            data: Vec::new(),
            name: String::from(name),
        }
    }

    pub fn add_data(&mut self, byte: u8) {
        self.data.push(byte);
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn set_name(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }
}

pub extern "C" fn crucible_ffi_create(name: *const u8, name_len: usize) -> *mut CrucibleFFI {
    let c_str = unsafe { core::slice::from_raw_parts(name, name_len) };
    let name_str = String::from_utf8_lossy(c_str).into_owned();
    Box::into_raw(Box::new(CrucibleFFI::new(&name_str)))
}

pub extern "C" fn crucible_ffi_destroy(ptr: *mut CrucibleFFI) {
    if !ptr.is_null() {
        unsafe { drop(Box::from_raw(ptr)) };
    }
}

pub extern "C" fn crucible_ffi_add_data(ptr: *mut CrucibleFFI, byte: u8) {
    if let Some(ref mut ffi) = unsafe { ptr.as_mut() } {
        ffi.add_data(byte);
    }
}

pub extern "C" fn crucible_ffi_get_data_len(ptr: *const CrucibleFFI) -> usize {
    if let Some(ffi) = unsafe { ptr.as_ref() } {
        ffi.get_data().len()
    } else {
        0
    }
}

pub extern "C" fn crucible_ffi_set_name(ptr: *mut CrucibleFFI, name: *const u8, name_len: usize) {
    if let Some(ref mut ffi) = unsafe { ptr.as_mut() } {
        let c_str = unsafe { core::slice::from_raw_parts(name, name_len) };
        let name_str = String::from_utf8_lossy(c_str).into_owned();
        ffi.set_name(&name_str);
    }
}

pub extern "C" fn crucible_ffi_get_name(ptr: *const CrucibleFFI, buf: *mut u8, buf_len: usize) -> usize {
    if let Some(ffi) = unsafe { ptr.as_ref() } {
        let name = ffi.get_name();
        let len = core::cmp::min(name.len(), buf_len);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), buf, len);
        }
        len
    } else {
        0
    }
}

pub extern "C" fn crucible_ffi_clear_data(ptr: *mut CrucibleFFI) {
    if let Some(ref mut ffi) = unsafe { ptr.as_mut() } {
        ffi.clear_data();
    }
}
