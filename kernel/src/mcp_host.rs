extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    let mut mcp = MCPHost::new();
    mcp.add_device("device1");
    mcp.add_device("device2");
    mcp.remove_device("device1");
    mcp.list_devices();
    if mcp.is_device_connected("device2") {
    } else {
    }
}

pub struct MCPHost {
    devices: Vec<String>,
}

impl MCPHost {
    pub fn new() -> Self {
        MCPHost {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device_name: &str) {
        if !self.devices.contains(&String::from(device_name)) {
            self.devices.push(String::from(device_name));
        }
    }

    pub fn remove_device(&mut self, device_name: &str) {
        self.devices.retain(|d| d != device_name);
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.clone()
    }

    pub fn is_device_connected(&self, device_name: &str) -> bool {
        self.devices.contains(&String::from(device_name))
    }
}
