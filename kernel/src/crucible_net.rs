extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn crucible_net_init() {
    // Initialization logic for the network module
}

pub extern "C" fn crucible_net_exit() {
    // Cleanup logic for the network module
}

pub struct NetworkInterface {
    name: String,
    ip_address: String,
    mac_address: [u8; 6],
    packets_received: u32,
    packets_sent: u32,
}

impl NetworkInterface {
    pub fn new(name: &str, ip_address: &str, mac_address: &[u8]) -> Self {
        NetworkInterface {
            name: String::from(name),
            ip_address: String::from(ip_address),
            mac_address: mac_address.try_into().unwrap(),
            packets_received: 0,
            packets_sent: 0,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_ip_address(&self) -> &str {
        &self.ip_address
    }

    pub fn get_mac_address(&self) -> &[u8] {
        &self.mac_address
    }

    pub fn receive_packet(&mut self, packet: &[u8]) -> bool {
        // Simulate receiving a packet
        if packet.len() > 0 {
            self.packets_received += 1;
            true
        } else {
            false
        }
    }

    pub fn send_packet(&mut self, packet: &[u8]) -> bool {
        // Simulate sending a packet
        if packet.len() > 0 {
            self.packets_sent += 1;
            true
        } else {
            false
        }
    }

    pub fn get_statistics(&self) -> (u32, u32) {
        (self.packets_received, self.packets_sent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_interface() {
        let mut iface = NetworkInterface::new("eth0", "192.168.1.1", &[0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]);
        assert_eq!(iface.get_name(), "eth0");
        assert_eq!(iface.get_ip_address(), "192.168.1.1");
        assert_eq!(iface.get_mac_address(), &[0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]);

        let packet = vec![0x01, 0x02, 0x03];
        assert!(iface.receive_packet(&packet));
        assert_eq!(iface.get_statistics(), (1, 0));

        assert!(iface.send_packet(&packet));
        assert_eq!(iface.get_statistics(), (1, 1));
    }
}
