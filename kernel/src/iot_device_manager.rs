extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut manager = IotDeviceManager::new();
    manager.register_device("sensor1".into(), DeviceType::Sensor);
    manager.register_device("actuator1".into(), DeviceType::Actuator);
    manager.enable_device("sensor1");
    manager.disable_device("actuator1");
    let status = manager.get_device_status("sensor1");
    println!("Device sensor1 status: {:?}", status);
}

#[derive(Debug)]
pub enum DeviceType {
    Sensor,
    Actuator,
}

#[derive(Debug)]
pub struct Device {
    name: String,
    device_type: DeviceType,
    is_enabled: bool,
}

impl Device {
    fn new(name: String, device_type: DeviceType) -> Self {
        Device {
            name,
            device_type,
            is_enabled: false,
        }
    }

    fn enable(&mut self) {
        self.is_enabled = true;
    }

    fn disable(&mut self) {
        self.is_enabled = false;
    }

    fn get_status(&self) -> (String, bool) {
        (self.name.clone(), self.is_enabled)
    }
}

pub struct IotDeviceManager {
    devices: Vec<Device>,
}

impl IotDeviceManager {
    pub fn new() -> Self {
        IotDeviceManager {
            devices: Vec::new(),
        }
    }

    pub fn register_device(&mut self, name: String, device_type: DeviceType) {
        let device = Device::new(name, device_type);
        self.devices.push(device);
    }

    pub fn enable_device(&mut self, name: &str) {
        if let Some(device) = self.devices.iter_mut().find(|d| d.name == name) {
            device.enable();
        }
    }

    pub fn disable_device(&mut self, name: &str) {
        if let Some(device) = self.devices.iter_mut().find(|d| d.name == name) {
            device.disable();
        }
    }

    pub fn get_device_status(&self, name: &str) -> Option<(String, bool)> {
        self.devices
            .iter()
            .find(|d| d.name == name)
            .map(|d| d.get_status())
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.iter().map(|d| d.name.clone()).collect()
    }
}
