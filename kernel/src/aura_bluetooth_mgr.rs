extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

pub extern "C" fn rust_ffi_exit() {
    // Clean up the module
}

struct BluetoothManager {
    devices: Vec<String>,
    connected_devices: Vec<String>,
}

impl BluetoothManager {
    pub fn new() -> Self {
        BluetoothManager {
            devices: Vec::new(),
            connected_devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        if !self.devices.contains(&device_name.to_string()) {
            self.devices.push(device_name.to_string());
        }
    }

    pub fn remove_device(&mut self, device_name: &str) {
        if let Some(index) = self.devices.iter().position(|d| d == device_name) {
            self.devices.remove(index);
        }
    }

    pub fn connect_device(&mut self, device_name: &str) -> bool {
        if self.devices.contains(&device_name.to_string()) && !self.connected_devices.contains(&device_name.to_string()) {
            self.connected_devices.push(device_name.to_string());
            true
        } else {
            false
        }
    }

    pub fn disconnect_device(&mut self, device_name: &str) -> bool {
        if let Some(index) = self.connected_devices.iter().position(|d| d == device_name) {
            self.connected_devices.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_connected_devices(&self) -> Vec<String> {
        self.connected_devices.clone()
    }
}

pub extern "C" fn aura_bluetooth_mgr_new() -> *mut BluetoothManager {
    Box::into_raw(Box::new(BluetoothManager::new()))
}

pub extern "C" fn aura_bluetooth_mgr_add_device(manager: *mut BluetoothManager, device_name: *const u8, len: usize) {
    unsafe {
        let slice = core::slice::from_raw_parts(device_name, len);
        if let Ok(name) = core::str::from_utf8(slice) {
            (*manager).add_device(name);
        }
    }
}

pub extern "C" fn aura_bluetooth_mgr_remove_device(manager: *mut BluetoothManager, device_name: *const u8, len: usize) {
    unsafe {
        let slice = core::slice::from_raw_parts(device_name, len);
        if let Ok(name) = core::str::from_utf8(slice) {
            (*manager).remove_device(name);
        }
    }
}

pub extern "C" fn aura_bluetooth_mgr_connect_device(manager: *mut BluetoothManager, device_name: *const u8, len: usize) -> bool {
    unsafe {
        let slice = core::slice::from_raw_parts(device_name, len);
        if let Ok(name) = core::str::from_utf8(slice) {
            (*manager).connect_device(name)
        } else {
            false
        }
    }
}

pub extern "C" fn aura_bluetooth_mgr_disconnect_device(manager: *mut BluetoothManager, device_name: *const u8, len: usize) -> bool {
    unsafe {
        let slice = core::slice::from_raw_parts(device_name, len);
        if let Ok(name) = core::str::from_utf8(slice) {
            (*manager).disconnect_device(name)
        } else {
            false
        }
    }
}

pub extern "C" fn aura_bluetooth_mgr_list_connected_devices(manager: *mut BluetoothManager, count: *mut usize) -> *const *const u8 {
    unsafe {
        let connected_devices = (*manager).list_connected_devices();
        let mut c_strings: Vec<*const u8> = connected_devices.iter().map(|s| s.as_ptr()).collect();
        *count = c_strings.len();
        c_strings.leak() as *const *const u8
    }
}

pub extern "C" fn aura_bluetooth_mgr_free(manager: *mut BluetoothManager) {
    unsafe {
        drop(Box::from_raw(manager));
    }
}
