extern crate alloc;
use alloc::vec::Vec;
use alloc::vec;

pub struct IpAddr(pub [u8; 4]);
pub struct MacAddr(pub [u8; 6]);

pub struct NetworkInterface {
    pub name: [u8; 8],
    pub mac: MacAddr,
    pub ip: IpAddr,
    pub subnet: IpAddr,
    pub gateway: IpAddr,
    pub mtu: u16,
    pub up: bool,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64
}

pub struct IpPacket {
    pub version: u8,
    pub ttl: u8,
    pub protocol: u8,
    pub src: IpAddr,
    pub dst: IpAddr,
    pub payload: Vec<u8>
}

impl IpPacket {
    pub fn new(src: [u8; 4], dst: [u8; 4], protocol: u8, payload: Vec<u8>) -> Self {
        Self {
            version: 4,
            ttl: 64,
            protocol,
            src: IpAddr(src),
            dst: IpAddr(dst),
            payload
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let total_len = 20 + self.payload.len();
        let mut pkt = vec![0u8; 20];
        pkt[0] = 0x45;
        pkt[1] = 0;
        pkt[2] = (total_len >> 8) as u8;
        pkt[3] = total_len as u8;
        pkt[8] = self.ttl;
        pkt[9] = self.protocol;
        pkt[12..16].copy_from_slice(&self.src.0);
        pkt[16..20].copy_from_slice(&self.dst.0);
        let cs = ip_checksum(&pkt[..20]);
        pkt[10..12].copy_from_slice(&cs.to_be_bytes());
        pkt.extend_from_slice(&self.payload);
        pkt
    }

    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 20 {
            return None;
        }
        let mut src = [0u8; 4];
        let mut dst = [0u8; 4];
        src.copy_from_slice(&data[12..16]);
        dst.copy_from_slice(&data[16..20]);
        let header_len = ((data[0] & 0x0F) * 4) as usize;
        Some(Self {
            version: data[0] >> 4,
            ttl: data[8],
            protocol: data[9],
            src: IpAddr(src),
            dst: IpAddr(dst),
            payload: data[header_len..].to_vec()
        })
    }
}

fn ip_checksum(data: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    for i in (0..data.len()).step_by(2) {
        let w = if i + 1 < data.len() {
            ((data[i] as u32) << 8) | data[i+1] as u32
        } else {
            (data[i] as u32) << 8
        };
        sum += w;
    }
    while sum >> 16 != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    !(sum as u16)
}

pub struct NetworkStack {
    pub interfaces: Vec<NetworkInterface>
}

impl NetworkStack {
    pub fn new() -> Self {
        Self {
            interfaces: Vec::new()
        }
    }

    pub fn add_interface(&mut self, iface: NetworkInterface) {
        self.interfaces.push(iface);
    }

    pub fn default_interface(&self) -> Option<&NetworkInterface> {
        self.interfaces.iter().find(|i| i.up)
    }
}
