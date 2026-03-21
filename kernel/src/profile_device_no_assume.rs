extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn profile_device_no_assume_init() {
    // Initialization logic for the module
}

pub extern "C" fn profile_device_no_assume_exit() {
    // Cleanup logic for the module
}

pub struct ProfileDevice {
    name: String,
    data: Vec<u8>,
    enabled: bool,
}

impl ProfileDevice {
    pub fn new(name: &str) -> Self {
        ProfileDevice {
            name: String::from(name),
            data: Vec::new(),
            enabled: false,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn collect_data(&mut self, data: &[u8]) {
        if self.enabled {
            self.data.extend_from_slice(data);
        }
    }

    pub fn get_collected_data(&self) -> &[u8] {
        &self.data
    }
}

pub extern "C" fn profile_device_no_assume_create(name: *const u8, name_len: usize) -> *mut ProfileDevice {
    let name_slice = unsafe { core::slice::from_raw_parts(name, name_len) };
    let device_name = String::from_utf8_lossy(name_slice).into_owned();
    Box::leak(Box::new(ProfileDevice::new(&device_name)))
}

pub extern "C" fn profile_device_no_assume_enable(device: *mut ProfileDevice) {
    unsafe { (*device).enable() };
}

pub extern "C" fn profile_device_no_assume_disable(device: *mut ProfileDevice) {
    unsafe { (*device).disable() };
}

pub extern "C" fn profile_device_no_assume_is_enabled(device: *const ProfileDevice) -> bool {
    unsafe { (*device).is_enabled() }
}

pub extern "C" fn profile_device_no_assume_collect_data(device: *mut ProfileDevice, data: *const u8, data_len: usize) {
    let data_slice = unsafe { core::slice::from_raw_parts(data, data_len) };
    unsafe { (*device).collect_data(data_slice) };
}

pub extern "C" fn profile_device_no_assume_get_collected_data(device: *const ProfileDevice, buffer: *mut u8, buffer_len: usize) -> usize {
    let collected_data = unsafe { (*device).get_collected_data() };
    let len_to_copy = core::cmp::min(collected_data.len(), buffer_len);
    unsafe { core::ptr::copy_nonoverlapping(collected_data.as_ptr(), buffer, len_to_copy) };
    len_to_copy
}
