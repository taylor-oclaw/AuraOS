extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum WifiSecurity {
    Open,
    Wep,
    Wpa2,
    Wpa3,
    Enterprise,
}

pub enum WifiState {
    Disconnected,
    Scanning,
    Connecting,
    Connected,
    Failed(String),
}

pub struct WifiNetwork {
    pub ssid: String,
    pub bssid: [u8; 6],
    pub signal_strength: i8,
    pub security: WifiSecurity,
    pub channel: u8,
    pub frequency_mhz: u32,
}

pub struct WifiConnection {
    pub network: WifiNetwork,
    pub ip_address: [u8; 4],
    pub gateway: [u8; 4],
    pub dns: [u8; 4],
    pub connected_at: u64,
}

pub struct WifiManager {
    pub state: WifiState,
    pub available_networks: Vec<WifiNetwork>,
    pub current_connection: Option<WifiConnection>,
    pub saved_networks: Vec<String>,
    pub scan_results_age: u64,
}

impl WifiManager {
    pub fn new() -> Self {
        Self {
            state: WifiState::Disconnected,
            available_networks: Vec::new(),
            current_connection: None,
            saved_networks: Vec::new(),
            scan_results_age: 0,
        }
    }

    pub fn scan(&mut self) {
        self.state = WifiState::Scanning;
        self.available_networks.clear();
        self.scan_results_age = 0;
    }

    pub fn connect(&mut self, ssid: &str, _password: &str) -> bool {
        if let Some(net) = self.available_networks.iter().find(|n| n.ssid == ssid) {
            self.state = WifiState::Connecting;
            self.current_connection = Some(WifiConnection {
                network: WifiNetwork {
                    ssid: net.ssid.clone(),
                    bssid: net.bssid,
                    signal_strength: net.signal_strength,
                    security: WifiSecurity::Wpa2,
                    channel: net.channel,
                    frequency_mhz: net.frequency_mhz,
                },
                ip_address: [192, 168, 1, 100],
                gateway: [192, 168, 1, 1],
                dns: [8, 8, 8, 8],
                connected_at: 0,
            };
            self.state = WifiState::Connected;
            if !self.saved_networks.contains(&String::from(ssid)) {
                self.saved_networks.push(String::from(ssid));
            }
            true
        } else {
            self.state = WifiState::Failed(String::from("Network not found"));
            false
        }
    }

    pub fn disconnect(&mut self) {
        self.current_connection = None;
        self.state = WifiState::Disconnected;
    }

    pub fn is_connected(&self) -> bool {
        matches!(self.state, WifiState::Connected)
    }

    pub fn signal_strength(&self) -> Option<i8> {
        self.current_connection.as_ref().map(|c| c.network.signal_strength)
    }
)}
