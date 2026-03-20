extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct Wifi6eHandler {
    // Example fields for a wifi handler module
    interface_name: String,
    connected_devices: Vec<String>,
    channel: u8,
    ssid: String,
    is_enabled: bool,
}

impl Wifi6eHandler {
    pub fn new(interface_name: &str, ssid: &str) -> Self {
        Wifi6eHandler {
            interface_name: String::from(interface_name),
            connected_devices: Vec::new(),
            channel: 1, // Default channel
            ssid: String::from(ssid),
            is_enabled: false,
        }
    }

    pub fn enable(&mut self) {
        self.is_enabled = true;
        // Simulate enabling the wifi interface
        println!("Wifi6eHandler enabled on {}", self.interface_name);
    }

    pub fn disable(&mut self) {
        self.is_enabled = false;
        // Simulate disabling the wifi interface
        println!("Wifi6eHandler disabled on {}", self.interface_name);
    }

    pub fn set_channel(&mut self, channel: u8) {
        if channel > 0 && channel < 14 {
            self.channel = channel;
            // Simulate setting the wifi channel
            println!("Channel set to {} on {}", self.channel, self.interface_name);
        } else {
            println!("Invalid channel number");
        }
    }

    pub fn connect_device(&mut self, device_mac: &str) {
        let mac_str = String::from(device_mac);
        if !self.connected_devices.contains(&mac_str) {
            self.connected_devices.push(mac_str);
            // Simulate connecting a device
            println!("Device {} connected to {}", device_mac, self.interface_name);
        } else {
            println!("Device already connected");
        }
    }

    pub fn disconnect_device(&mut self, device_mac: &str) {
        if let Some(index) = self.connected_devices.iter().position(|x| x == device_mac) {
            self.connected_devices.remove(index);
            // Simulate disconnecting a device
            println!("Device {} disconnected from {}", device_mac, self.interface_name);
        } else {
            println!("Device not found");
        }
    }

    pub fn list_connected_devices(&self) -> Vec<String> {
        self.connected_devices.clone()
    }
}
