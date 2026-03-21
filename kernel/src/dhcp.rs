extern crate alloc;
use alloc::vec::Vec;
use alloc::vec;

pub struct DhcpConfig {
    pub ip: [u8; 4],
    pub subnet: [u8; 4],
    pub gateway: [u8; 4],
    pub dns: [u8; 4],
    pub lease_time: u32,
}

#[derive(PartialEq)]
pub enum DhcpState {
    Init,
    Selecting,
    Requesting,
    Bound,
    Renewing,
}

pub struct DhcpClient {
    pub state: DhcpState,
    pub xid: u32,
    pub config: Option<DhcpConfig>,
}

impl DhcpClient {
    pub fn new() -> Self {
        DhcpClient {
            state: DhcpState::Init,
            xid: 0xABCD1234,
            config: None,
        }
    }

    pub fn build_discover(&self) -> Vec<u8> {
        let mut packet = vec![0; 300];
        // DHCP header
        packet[0..4].copy_from_slice(&[1, 1, 6, 0]); // op, htype, hlen, hops
        packet[4..8].copy_from_slice(&self.xid.to_be_bytes()); // xid
        packet[24..28].copy_from_slice(&[0; 4]); // ciaddr
        packet[28..32].copy_from_slice(&[0; 4]); // yiaddr
        packet[32..36].copy_from_slice(&[0; 4]); // siaddr
        packet[36..40].copy_from_slice(&[0; 4]); // giaddr
        packet[40..44].copy_from_slice(&[0; 4]); // chaddr

        // Magic cookie
        packet[236..240].copy_from_slice(&[99, 130, 83, 99]);

        // DHCP options
        packet[240] = 53; // Option: DHCP Message Type
        packet[241] = 1;  // Length: 1 byte
        packet[242] = 1;  // Discover message

        packet[243] = 255; // End of options

        packet
    }

    pub fn build_request(&self, server_ip: [u8; 4], offered_ip: [u8; 4]) -> Vec<u8> {
        let mut packet = vec![0; 300];
        // DHCP header
        packet[0..4].copy_from_slice(&[1, 1, 6, 0]); // op, htype, hlen, hops
        packet[4..8].copy_from_slice(&self.xid.to_be_bytes()); // xid
        packet[24..28].copy_from_slice(&offered_ip); // ciaddr
        packet[28..32].copy_from_slice(&[0; 4]); // yiaddr
        packet[32..36].copy_from_slice(&server_ip); // siaddr
        packet[36..40].copy_from_slice(&[0; 4]); // giaddr
        packet[40..44].copy_from_slice(&[0; 4]); // chaddr

        // Magic cookie
        packet[236..240].copy_from_slice(&[99, 130, 83, 99]);

        // DHCP options
        packet[240] = 53; // Option: DHCP Message Type
        packet[241] = 1;  // Length: 1 byte
        packet[242] = 3;  // Request message

        packet[243] = 50; // Option: Requested IP Address
        packet[244] = 4;  // Length: 4 bytes
        packet[245..249].copy_from_slice(&offered_ip);

        packet[249] = 255; // End of options

        packet
    }

    pub fn parse_offer(&mut self, data: &[u8]) -> Option<DhcpConfig> {
        if data.len() < 300 || &data[236..240] != &[99, 130, 83, 99] {
            return None;
        }

        let mut ip = [0; 4];
        let mut subnet = [0; 4];
        let mut gateway = [0; 4];
        let mut dns = [0; 4];
        let mut lease_time = 0;

        let mut i = 240;
        while i < data.len() {
            if data[i] == 53 && data[i + 1] == 1 && data[i + 2] == 2 {
                // Offer message
                break;
            } else if data[i] == 50 && data[i + 1] == 4 {
                ip.copy_from_slice(&data[i + 2..i + 6]);
            } else if data[i] == 1 && data[i + 1] == 4 {
                subnet.copy_from_slice(&data[i + 2..i + 6]);
            } else if data[i] == 3 && data[i + 1] == 4 {
                gateway.copy_from_slice(&data[i + 2..i + 6]);
            } else if data[i] == 6 && data[i + 1] == 4 {
                dns.copy_from_slice(&data[i + 2..i + 6]);
            } else if data[i] == 51 && data[i + 1] == 4 {
                lease_time = u32::from_be_bytes([data[i + 2], data[i + 3], data[i + 4], data[i + 5]]);
            }
            i += (data[i + 1] as usize) + 2;
        }

        Some(DhcpConfig {
            ip,
            subnet,
            gateway,
            dns,
            lease_time,
        })
    }

    pub fn is_bound(&self) -> bool {
        self.state == DhcpState::Bound
    }
}
