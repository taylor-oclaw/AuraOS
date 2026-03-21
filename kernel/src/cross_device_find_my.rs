extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct CrossDeviceFindMy {
    devices: Vec<String>,
}

impl CrossDeviceFindMy {
    pub fn new() -> Self {
        CrossDeviceFindMy {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        self.devices.push(String::from(device_name));
    }

    pub fn remove_device(&mut self, device_name: &str) {
        if let Some(index) = self.devices.iter().position(|d| d == device_name) {
            self.devices.remove(index);
        }
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.clone()
    }

    pub fn find_device(&self, device_name: &str) -> bool {
        self.devices.contains(&String::from(device_name))
    }

    pub fn count_devices(&self) -> usize {
        self.devices.len()
    }
}

#[no_mangle]
pub extern "C" fn cross_device_find_my_new() -> *mut CrossDeviceFindMy {
    Box::into_raw(Box::new(CrossDeviceFindMy::new()))
}

#[no_mangle]
pub extern "C" fn cross_device_find_my_add_device(module: *mut CrossDeviceFindMy, device_name: *const u8) {
    unsafe {
        let module = &mut *module;
        let c_str = core::ffi::CStr::from_ptr(device_name as *const _);
        if let Ok(s) = c_str.to_str() {
            module.add_device(s);
        }
    }
}

#[no_mangle]
pub extern "C" fn cross_device_find_my_remove_device(module: *mut CrossDeviceFindMy, device_name: *const u8) {
    unsafe {
        let module = &mut *module;
        let c_str = core::ffi::CStr::from_ptr(device_name as *const _);
        if let Ok(s) = c_str.to_str() {
            module.remove_device(s);
        }
    }
}

#[no_mangle]
pub extern "C" fn cross_device_find_my_list_devices(module: *mut CrossDeviceFindMy, devices: *mut *const u8, count: *mut usize) -> i32 {
    unsafe {
        let module = &*module;
        let device_names: Vec<String> = module.list_devices();
        let mut c_strings: Vec<*const u8> = device_names.iter().map(|s| s.as_ptr()).collect();

        *devices = c_strings.as_mut_ptr();
        *count = c_strings.len();
        core::mem::forget(c_strings); // Prevent deallocation
        0
    }
}

#[no_mangle]
pub extern "C" fn cross_device_find_my_find_device(module: *mut CrossDeviceFindMy, device_name: *const u8) -> i32 {
    unsafe {
        let module = &*module;
        let c_str = core::ffi::CStr::from_ptr(device_name as *const _);
        if let Ok(s) = c_str.to_str() {
            return module.find_device(s) as i32;
        }
        0
    }
}

#[no_mangle]
pub extern "C" fn cross_device_find_my_count_devices(module: *mut CrossDeviceFindMy) -> usize {
    unsafe {
        let module = &*module;
        module.count_devices()
    }
}
