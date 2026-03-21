extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum VpnProtocol {
    WireGuard,
    OpenVpn,
    IpSec,
}

pub enum VpnState {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

pub struct VpnConfig {
    pub name: String,
    pub server: String,
    pub port: u16,
    pub protocol: VpnProtocol,
    pub private_key: [u8; 32],
    pub dns: [u8; 4],
}

pub struct VpnClient {
    pub configs: Vec<VpnConfig>,
    pub state: VpnState,
    pub active_config: Option<usize>,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub connected_since: Option<u64>,
}

impl VpnClient {
    pub fn new() -> Self {
        Self {
            configs: Vec::new(),
            state: VpnState::Disconnected,
            active_config: None,
            bytes_sent: 0,
            bytes_received: 0,
            connected_since: None,
        }
    }

    pub fn add_config(&mut self, name: &str, server: &str, port: u16, proto: VpnProtocol) {
        self.configs.push(VpnConfig {
            name: String::from(name),
            server: String::from(server),
            port,
            protocol: proto,
            private_key: [0; 32],
            dns: [8, 8, 8, 8],
        });
    }

    pub fn connect(&mut self, idx: usize) -> bool {
        if idx < self.configs.len() {
            self.state = VpnState::Connected;
            self.active_config = Some(idx);
            self.connected_since = Some(0);
            true
        } else {
            false
        }
    }

    pub fn disconnect(&mut self) {
        self.state = VpnState::Disconnected;
        self.active_config = None;
        self.connected_since = None;
    }

    pub fn is_connected(&self) -> bool {
        matches!(self.state, VpnState::Connected)
    }
}
