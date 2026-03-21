extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshWakeOnLan {
    mac_addresses: Vec<String>,
}

impl MeshWakeOnLan {
    pub fn new() -> Self {
        MeshWakeOnLan {
            mac_addresses: Vec::new(),
        }
    }

    pub fn add_mac_address(&mut self, mac: &str) {
        if Self::is_valid_mac(mac) {
            self.mac_addresses.push(String::from(mac));
        }
    }

    pub fn remove_mac_address(&mut self, mac: &str) {
        self.mac_addresses.retain(|m| m != mac);
    }

    pub fn get_mac_addresses(&self) -> Vec<String> {
        self.mac_addresses.clone()
    }

    pub fn is_valid_mac(mac: &str) -> bool {
        let parts: Vec<&str> = mac.split(':').collect();
        if parts.len() != 6 {
            return false;
        }
        for part in parts {
            if part.len() != 2 || !part.chars().all(|c| c.is_digit(16)) {
                return false;
            }
        }
        true
    }

    pub fn wake_on_lan(&self, mac: &str) -> bool {
        if let Some(index) = self.mac_addresses.iter().position(|m| m == mac) {
            // Simulate sending a Wake-on-LAN packet
            println!("Sending WoL packet to {}", self.mac_addresses[index]);
            true
        } else {
            false
        }
    }
}
