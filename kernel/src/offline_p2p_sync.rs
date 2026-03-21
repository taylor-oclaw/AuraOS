extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct OfflineP2PSync {
    peers: Vec<String>,
    data_store: Vec<u8>,
}

impl OfflineP2PSync {
    pub fn new() -> Self {
        OfflineP2PSync {
            peers: Vec::new(),
            data_store: Vec::new(),
        }
    }

    pub fn add_peer(&mut self, peer_address: &str) {
        if !self.peers.contains(&peer_address.to_string()) {
            self.peers.push(peer_address.to_string());
        }
    }

    pub fn remove_peer(&mut self, peer_address: &str) {
        self.peers.retain(|p| p != peer_address);
    }

    pub fn list_peers(&self) -> Vec<String> {
        self.peers.clone()
    }

    pub fn store_data(&mut self, data: &[u8]) {
        self.data_store.extend_from_slice(data);
    }

    pub fn retrieve_data(&self) -> &[u8] {
        &self.data_store
    }
}
