extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut solar_system = SolarSystem::new();
    solar_system.add_device("SolarPanel1", 250);
    solar_system.add_device("Battery1", 300);
    solar_system.add_device("Inverter1", 150);

    println!("Total Power: {}W", solar_system.total_power());
    println!("Device Count: {}", solar_system.device_count());

    if let Some(device) = solar_system.get_device_by_name("SolarPanel1") {
        println!("Found device: {}", device.name);
    } else {
        println!("Device not found");
    }

    solar_system.remove_device("Battery1");

    println!("Total Power after removal: {}W", solar_system.total_power());
    println!("Device Count after removal: {}", solar_system.device_count());

    loop {}
}

pub struct SolarSystem {
    devices: Vec<Device>,
}

impl SolarSystem {
    pub fn new() -> Self {
        SolarSystem { devices: Vec::new() }
    }

    pub fn add_device(&mut self, name: &str, power_output: u32) {
        let device = Device {
            name: String::from(name),
            power_output,
        };
        self.devices.push(device);
    }

    pub fn remove_device(&mut self, name: &str) {
        self.devices.retain(|d| d.name != name);
    }

    pub fn total_power(&self) -> u32 {
        self.devices.iter().map(|d| d.power_output).sum()
    }

    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    pub fn get_device_by_name(&self, name: &str) -> Option<&Device> {
        self.devices.iter().find(|d| d.name == name)
    }
}

pub struct Device {
    name: String,
    power_output: u32,
}
