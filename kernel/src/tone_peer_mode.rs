extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn tone_peer_mode_init() {
    // Initialization logic for the module
}

pub extern "C" fn tone_peer_mode_exit() {
    // Cleanup logic for the module
}

pub struct TonePeerMode {
    peers: Vec<String>,
    mode: String,
}

impl TonePeerMode {
    pub fn new(mode: &str) -> Self {
        TonePeerMode {
            peers: Vec::new(),
            mode: String::from(mode),
        }
    }

    pub fn add_peer(&mut self, peer: &str) {
        self.peers.push(String::from(peer));
    }

    pub fn remove_peer(&mut self, peer: &str) -> bool {
        if let Some(index) = self.peers.iter().position(|p| p == peer) {
            self.peers.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_peers(&self) -> Vec<String> {
        self.peers.clone()
    }

    pub fn set_mode(&mut self, mode: &str) {
        self.mode = String::from(mode);
    }

    pub fn get_mode(&self) -> String {
        self.mode.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tone_peer_mode() {
        let mut tpm = TonePeerMode::new("default");
        assert_eq!(tpm.get_mode(), "default");

        tpm.add_peer("peer1");
        tpm.add_peer("peer2");
        assert_eq!(tpm.get_peers(), vec![String::from("peer1"), String::from("peer2")]);

        assert!(tpm.remove_peer("peer1"));
        assert!(!tpm.remove_peer("peer3"));
        assert_eq!(tpm.get_peers(), vec![String::from("peer2")]);

        tpm.set_mode("advanced");
        assert_eq!(tpm.get_mode(), "advanced");
    }
}
