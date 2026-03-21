extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut optimizer = SmartHomeEnergyOptimizer::new();
    optimizer.add_device("Living Room Light", 100);
    optimizer.add_device("Kitchen Appliance", 200);
    optimizer.add_device("Bedroom Fan", 50);

    optimizer.enable_device("Living Room Light");
    optimizer.disable_device("Kitchen Appliance");

    let total_usage = optimizer.calculate_total_usage();
    println!("Total Energy Usage: {} watts", total_usage);

    loop {}
}

pub struct SmartHomeEnergyOptimizer {
    devices: Vec<Device>,
}

impl SmartHomeEnergyOptimizer {
    pub fn new() -> Self {
        SmartHomeEnergyOptimizer {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, name: &str, power_usage: u32) {
        let device = Device {
            name: String::from(name),
            power_usage,
            is_enabled: false,
        };
        self.devices.push(device);
    }

    pub fn enable_device(&mut self, name: &str) {
        if let Some(device) = self.devices.iter_mut().find(|d| d.name == name) {
            device.is_enabled = true;
        }
    }

    pub fn disable_device(&mut self, name: &str) {
        if let Some(device) = self.devices.iter_mut().find(|d| d.name == name) {
            device.is_enabled = false;
        }
    }

    pub fn calculate_total_usage(&self) -> u32 {
        self.devices
            .iter()
            .filter(|&d| d.is_enabled)
            .map(|d| d.power_usage)
            .sum()
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.devices.iter().map(|d| d.name.clone()).collect()
    }
}

struct Device {
    name: String,
    power_usage: u32,
    is_enabled: bool,
}
