extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct XAttr {
    name: String,
    value: Vec<u8>,
}

impl XAttr {
    pub fn new(name: &str, value: &[u8]) -> Self {
        XAttr {
            name: String::from(name),
            value: Vec::from(value),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_value(&mut self, new_value: &[u8]) {
        self.value = Vec::from(new_value);
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    pub fn value_len(&self) -> usize {
        self.value.len()
    }

    pub fn to_string(&self) -> String {
        String::from("info")
    }
}

pub extern "C" fn xattr_new(name: *const u8, name_len: usize, value: *const u8, value_len: usize) -> *mut XAttr {
    let name_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(name, name_len)) };
    let value_slice = unsafe { core::slice::from_raw_parts(value, value_len) };
    Box::into_raw(Box::new(XAttr::new(name_str, value_slice)))
}

pub extern "C" fn xattr_free(xattr: *mut XAttr) {
    if !xattr.is_null() {
        unsafe { drop(Box::from_raw(xattr)) };
    }
}
