extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CellularModem {
    modem_id: String,
    signal_strength: u8,
    connected: bool,
    network_type: String,
    ip_address: Option<String>,
}

impl CellularModem {
    pub fn new(modem_id: &str) -> Self {
        CellularModem {
            modem_id: String::from(modem_id),
            signal_strength: 0,
            connected: false,
            network_type: String::new(),
            ip_address: None,
        }
    }

    pub fn connect(&mut self, network_type: &str) {
        // Simulate connecting to a network
        self.connected = true;
        self.network_type = String::from(network_type);
        self.signal_strength = 85; // Example signal strength
        self.ip_address = Some(String::from("192.168.1.100")); // Example IP address
    }

    pub fn disconnect(&mut self) {
        // Simulate disconnecting from the network
        self.connected = false;
        self.network_type.clear();
        self.signal_strength = 0;
        self.ip_address = None;
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn get_signal_strength(&self) -> u8 {
        self.signal_strength
    }

    pub fn get_ip_address(&self) -> Option<&str> {
        self.ip_address.as_deref()
    }
}
