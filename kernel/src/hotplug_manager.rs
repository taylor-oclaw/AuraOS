extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct HotplugManager {
    devices: Vec<String>,
}

impl HotplugManager {
    pub fn new() -> Self {
        HotplugManager {
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

    pub fn has_device(&self, device_name: &str) -> bool {
        self.devices.contains(&String::from(device_name))
    }

    pub fn count_devices(&self) -> usize {
        self.devices.len()
    }
}

pub extern "C" fn hotplug_manager_new() -> *mut HotplugManager {
    Box::into_raw(Box::new(HotplugManager::new()))
}

pub extern "C" fn hotplug_manager_add_device(manager: *mut HotplugManager, device_name: *const u8) {
    unsafe {
        let manager = &mut *manager;
        let c_str = core::ffi::CStr::from_ptr(device_name as *const i8);
        if let Ok(name) = c_str.to_str() {
            manager.add_device(name);
        }
    }
}

pub extern "C" fn hotplug_manager_remove_device(manager: *mut HotplugManager, device_name: *const u8) {
    unsafe {
        let manager = &mut *manager;
        let c_str = core::ffi::CStr::from_ptr(device_name as *const i8);
        if let Ok(name) = c_str.to_str() {
            manager.remove_device(name);
        }
    }
}

pub extern "C" fn hotplug_manager_list_devices(manager: *mut HotplugManager, devices: *mut *mut u8, count: *mut usize) -> i32 {
    unsafe {
        let manager = &*manager;
        let device_names = manager.list_devices();
        *count = device_names.len();

        if !devices.is_null() {
            for (i, name) in device_names.iter().enumerate() {
                let c_str = core::ffi::CString::new(name.as_bytes()).unwrap_or_else(|_| core::ffi::CString::new("").unwrap());
                (*devices).add(i * core::mem::size_of::<*mut u8>());
                *((*devices).offset(i as isize)) = c_str.into_raw();
            }
        }

        0
    }
}

pub extern "C" fn hotplug_manager_has_device(manager: *mut HotplugManager, device_name: *const u8) -> i32 {
    unsafe {
        let manager = &*manager;
        let c_str = core::ffi::CStr::from_ptr(device_name as *const i8);
        if let Ok(name) = c_str.to_str() {
            manager.has_device(name) as i32
        } else {
            0
        }
    }
}

pub extern "C" fn hotplug_manager_count_devices(manager: *mut HotplugManager) -> usize {
    unsafe {
        let manager = &*manager;
        manager.count_devices()
    }
}
