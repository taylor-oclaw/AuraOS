extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraNetworkMgr {
    network_interfaces: Vec<String>,
}

impl AuraNetworkMgr {
    pub fn new() -> Self {
        AuraNetworkMgr {
            network_interfaces: Vec::<String>::new(),
        }
    }

    pub fn add_network_interface(&mut self, interface_name: String) {
        self.network_interfaces.push(interface_name);
    }

    pub fn remove_network_interface(&mut self, interface_name: &str) {
        let index = self
            .network_interfaces
            .iter()
            .position(|x| x == interface_name)
            .unwrap_or_else(|| panic!("Interface not found"));
        self.network_interfaces.remove(index);
    }

    pub fn get_network_interfaces(&self) -> Vec<String> {
        self.network_interfaces.clone()
    }

    pub fn is_interface_up(&self, interface_name: &str) -> bool {
        let index = self
            .network_interfaces
            .iter()
            .position(|x| x == interface_name)
            .unwrap_or_else(|| panic!("Interface not found"));
        self.network_interfaces[index].len() > 0
    }

    pub fn get_interface_status(&self, interface_name: &str) -> String {
        let index = self
            .network_interfaces
            .iter()
            .position(|x| x == interface_name)
            .unwrap_or_else(|| panic!("Interface not found"));
        format!("{} is up", self.network_interfaces[index])
    }
}