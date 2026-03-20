extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct NetworkBridge {
    interfaces: Vec<String>,
    connected: bool,
}

impl NetworkBridge {
    pub fn new() -> Self {
        NetworkBridge {
            interfaces: Vec::new(),
            connected: false,
        }
    }

    pub fn add_interface(&mut self, interface_name: &str) {
        self.interfaces.push(interface_name.to_string());
    }

    pub fn remove_interface(&mut self, interface_name: &str) -> bool {
        if let Some(index) = self.interfaces.iter().position(|x| x == interface_name) {
            self.interfaces.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_interfaces(&self) -> Vec<String> {
        self.interfaces.clone()
    }

    pub fn connect(&mut self) {
        if !self.connected && !self.interfaces.is_empty() {
            self.connected = true;
        }
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }
}
