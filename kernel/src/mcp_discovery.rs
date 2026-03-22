extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct MCPDiscovery {
    devices: Vec<String>,
    discovered_count: usize,
}

impl MCPDiscovery {
    pub fn new() -> Self {
        MCPDiscovery {
            devices: Vec::new(),
            discovered_count: 0,
        }
    }

    pub fn discover_device(&mut self, device_name: &str) {
        let name = String::from(device_name);
        self.devices.push(name);
        self.discovered_count += 1;
    }

    pub fn get_discovered_devices(&self) -> &[String] {
        &self.devices
    }

    pub fn get_device_count(&self) -> usize {
        self.devices.len()
    }

    pub fn reset_discovery(&mut self) {
        self.devices.clear();
        self.discovered_count = 0;
    }
}