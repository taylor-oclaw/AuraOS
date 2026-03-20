extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn tgi_compat_init() -> i32 {
    0
}

pub extern "C" fn tgi_compat_exit() {}

struct TGICompat {
    data: Vec<u8>,
    name: String,
}

impl TGICompat {
    pub fn new(name: &str) -> Self {
        TGICompat {
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

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn set_name(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

pub extern "C" fn tgi_compat_create(name: *const u8, name_len: usize) -> *mut TGICompat {
    let c_str = unsafe { core::slice::from_raw_parts(name, name_len) };
    let name_str = String::from_utf8_lossy(c_str).into_owned();
    Box::leak(Box::new(TGICompat::new(&name_str)))
}

pub extern "C" fn tgi_compat_add_data(module: *mut TGICompat, byte: u8) {
    unsafe { (*module).add_data(byte); }
}

pub extern "C" fn tgi_compat_get_data(module: *const TGICompat, data_out: *mut u8, len: usize) -> usize {
    let module = unsafe { &*module };
    let data_len = module.get_data().len();
    if len > 0 && !data_out.is_null() {
        let copy_len = core::cmp::min(data_len, len);
        unsafe {
            core::ptr::copy_nonoverlapping(module.get_data().as_ptr(), data_out, copy_len);
        }
        copy_len
    } else {
        data_len
    }
}

pub extern "C" fn tgi_compat_clear_data(module: *mut TGICompat) {
    unsafe { (*module).clear_data(); }
}

pub extern "C" fn tgi_compat_set_name(module: *mut TGICompat, name: *const u8, name_len: usize) {
    let c_str = unsafe { core::slice::from_raw_parts(name, name_len) };
    let name_str = String::from_utf8_lossy(c_str).into_owned();
    unsafe { (*module).set_name(&name_str); }
}

pub extern "C" fn tgi_compat_get_name(module: *const TGICompat, name_out: *mut u8, len: usize) -> usize {
    let module = unsafe { &*module };
    let name_len = module.get_name().len();
    if len > 0 && !name_out.is_null() {
        let copy_len = core::cmp::min(name_len, len);
        unsafe {
            core::ptr::copy_nonoverlapping(module.get_name().as_ptr(), name_out, copy_len);
        }
        copy_len
    } else {
        name_len
    }
}

pub extern "C" fn tgi_compat_destroy(module: *mut TGICompat) {
    unsafe { drop(Box::from_raw(module)); }
}
