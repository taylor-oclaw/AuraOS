extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let monitor = SmartHomeEnergyMonitor::new();
    monitor.initialize();
    loop {}
}

pub struct SmartHomeEnergyMonitor {
    devices: Vec<String>,
    total_energy_consumed: u32,
    peak_power_usage: u32,
    current_power_usage: u32,
    is_active: bool,
}

impl SmartHomeEnergyMonitor {
    pub fn new() -> Self {
        SmartHomeEnergyMonitor {
            devices: Vec::new(),
            total_energy_consumed: 0,
            peak_power_usage: 0,
            current_power_usage: 0,
            is_active: false,
        }
    }

    pub fn initialize(&mut self) {
        // Initialize the energy monitor
        self.is_active = true;
    }

    pub fn add_device(&mut self, device_name: &str) {
        if self.is_active {
            self.devices.push(String::from(device_name));
        } else {
        }
    }

    pub fn update_power_usage(&mut self, power_usage: u32) {
        if self.is_active {
            self.current_power_usage = power_usage;
            if power_usage > self.peak_power_usage {
                self.peak_power_usage = power_usage;
            }
            self.total_energy_consumed += power_usage;
        } else {
        }
    }

    pub fn get_total_energy_consumed(&self) -> u32 {
        if self.is_active {
            self.total_energy_consumed
        } else {
            0
        }
    }

    pub fn get_peak_power_usage(&self) -> u32 {
        if self.is_active {
            self.peak_power_usage
        } else {
            0
        }
    }
}
