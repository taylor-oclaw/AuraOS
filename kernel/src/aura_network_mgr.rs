extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraNetworkMgr {
    interfaces: Vec<String>,
    connected_peers: Vec<String>,
    network_config: String,
}

impl AuraNetworkMgr {
    pub fn new() -> Self {
        AuraNetworkMgr {
            interfaces: Vec::new(),
            connected_peers: Vec::new(),
            network_config: String::from("default"),
        }
    }

    pub fn add_interface(&mut self, interface_name: &str) {
        self.interfaces.push(String::from(interface_name));
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

    pub fn connect_to_peer(&mut self, peer_address: &str) {
        self.connected_peers.push(String::from(peer_address));
    }

    pub fn disconnect_from_peer(&mut self, peer_address: &str) -> bool {
        if let Some(index) = self.connected_peers.iter().position(|x| x == peer_address) {
            self.connected_peers.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_connected_peers(&self) -> Vec<String> {
        self.connected_peers.clone()
    }

    pub fn set_network_config(&mut self, config: &str) {
        self.network_config = String::from(config);
    }

    pub fn get_network_config(&self) -> String {
        self.network_config.clone()
    }
}
