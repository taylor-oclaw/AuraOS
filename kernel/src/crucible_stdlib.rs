extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn crucible_stdlib_init() {
    // Initialization code for the module
}

pub extern "C" fn crucible_stdlib_exit() {
    // Cleanup code for the module
}

pub struct KernelString {
    data: Vec<u8>,
}

impl KernelString {
    pub fn new() -> Self {
        KernelString { data: Vec::new() }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        KernelString {
            data: bytes.to_vec(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn push_str(&mut self, other: &[u8]) {
        self.data.extend_from_slice(other);
    }
}

pub extern "C" fn crucible_stdlib_create_string() -> *const KernelString {
    Box::into_raw(Box::new(KernelString::new()))
}

pub extern "C" fn crucible_stdlib_destroy_string(ptr: *mut KernelString) {
    if !ptr.is_null() {
        unsafe { drop(Box::from_raw(ptr)) };
    }
}

pub extern "C" fn crucible_stdlib_string_len(ptr: *const KernelString) -> usize {
    if ptr.is_null() {
        0
    } else {
        unsafe { (*ptr).len() }
    }
}

pub extern "C" fn crucible_stdlib_string_is_empty(ptr: *const KernelString) -> bool {
    if ptr.is_null() {
        true
    } else {
        unsafe { (*ptr).is_empty() }
    }
}

pub extern "C" fn crucible_stdlib_string_as_bytes(ptr: *const KernelString, len: *mut usize) -> *const u8 {
    if ptr.is_null() {
        core::ptr::null()
    } else {
        unsafe {
            let bytes = (*ptr).as_bytes();
            *len = bytes.len();
            bytes.as_ptr()
        }
    }
}

pub extern "C" fn crucible_stdlib_string_push_str(
    ptr: *mut KernelString,
    other: *const u8,
    len: usize,
 {
    if !ptr.is_null() && !other.is_null() {
        unsafe {
            let bytes = core::slice::from_raw_parts(other, len);
            (*ptr).push_str(bytes);
        }
    }
}
