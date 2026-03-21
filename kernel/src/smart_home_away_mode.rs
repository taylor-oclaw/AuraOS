extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut away_mode = SmartHomeAwayMode::new();
    away_mode.enable_away_mode();
    away_mode.set_device_status("lights", false);
    away_mode.set_device_status("thermostat", 20.0);
    away_mode.add_exception_device("security_system");
    away_mode.log_status();
    loop {}
}

pub struct SmartHomeAwayMode {
    enabled: bool,
    device_statuses: Vec<(String, DeviceStatus)>,
    exception_devices: Vec<String>,
}

impl SmartHomeAwayMode {
    pub fn new() -> Self {
        SmartHomeAwayMode {
            enabled: false,
            device_statuses: Vec::new(),
            exception_devices: Vec::new(),
        }
    }

    pub fn enable_away_mode(&mut self) {
        self.enabled = true;
    }

    pub fn disable_away_mode(&mut self) {
        self.enabled = false;
    }

    pub fn set_device_status(&mut self, device_name: &str, status: DeviceStatus) {
        if let Some(device) = self.device_statuses.iter_mut().find(|d| d.0 == device_name) {
            device.1 = status;
        } else {
            self.device_statuses.push((String::from(device_name), status));
        }
    }

    pub fn add_exception_device(&mut self, device_name: &str) {
        if !self.exception_devices.contains(&device_name.to_string()) {
            self.exception_devices.push(String::from(device_name));
        } else {
        }
    }

    pub fn log_status(&self) {
        for (device, status) in &self.device_statuses {
        }
    }
}

#[derive(Debug)]
pub enum DeviceStatus {
    On,
    Off,
    Temperature(f32),
}
