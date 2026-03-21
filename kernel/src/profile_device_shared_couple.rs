extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct ProfileDeviceSharedCouple {
    name: String,
    data: Vec<u8>,
    counter: u32,
    active: bool,
    status: String,
}

impl ProfileDeviceSharedCouple {
    pub fn new(name: &str) -> Self {
        ProfileDeviceSharedCouple {
            name: String::from(name),
            data: Vec::new(),
            counter: 0,
            active: false,
            status: String::from("inactive"),
        }
    }

    pub fn activate(&mut self) {
        if !self.active {
            self.active = true;
            self.status = String::from("active");
        }
    }

    pub fn deactivate(&mut self) {
        if self.active {
            self.active = false;
            self.status = String::from("inactive");
        }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
        self.counter += 1;
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }

    pub fn reset_counter(&mut self) {
        self.counter = 0;
    }
}

pub extern "C" fn profile_device_shared_couple_new(name: *const u8, name_len: usize) -> *mut ProfileDeviceSharedCouple {
    let name_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(name, name_len)) };
    Box::into_raw(Box::new(ProfileDeviceSharedCouple::new(name_str)))
}

pub extern "C" fn profile_device_shared_couple_activate(ptr: *mut ProfileDeviceSharedCouple) {
    unsafe { (*ptr).activate() }
}

pub extern "C" fn profile_device_shared_couple_deactivate(ptr: *mut ProfileDeviceSharedCouple) {
    unsafe { (*ptr).deactivate() }
}

pub extern "C" fn profile_device_shared_couple_add_data(ptr: *mut ProfileDeviceSharedCouple, data: *const u8, len: usize) {
    let data_slice = unsafe { core::slice::from_raw_parts(data, len) };
    unsafe { (*ptr).add_data(data_slice) }
}

pub extern "C" fn profile_device_shared_couple_get_status(ptr: *mut ProfileDeviceSharedCouple) -> *const u8 {
    let status = unsafe { (*ptr).get_status() };
    status.as_ptr()
}

pub extern "C" fn profile_device_shared_couple_reset_counter(ptr: *mut ProfileDeviceSharedCouple) {
    unsafe { (*ptr).reset_counter() }
}
