extern crate alloc;
use alloc::vec::Vec;

pub struct EthernetFrame {
    pub dst_mac: [u8; 6],
    pub src_mac: [u8; 6],
    pub ethertype: u16,
    pub payload: Vec<u8>,
}

impl EthernetFrame {
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 14 {
            return None;
        }
        let mut dst = [0u8; 6];
        let mut src = [0u8; 6];
        dst.copy_from_slice(&data[0..6]);
        src.copy_from_slice(&data[6..12]);
        let ethertype = u16::from_be_bytes([data[12], data[13]]);
        Some(Self {
            dst_mac: dst,
            src_mac: src,
            ethertype,
            payload: data[14..].to_vec(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut frame = Vec::new();
        frame.extend_from_slice(&self.dst_mac);
        frame.extend_from_slice(&self.src_mac);
        frame.extend_from_slice(&self.ethertype.to_be_bytes());
        frame.extend_from_slice(&self.payload);
        frame
    }

    pub fn is_ipv4(&self) -> bool {
        self.ethertype == 0x0800
    }

    pub fn is_arp(&self) -> bool {
        self.ethertype == 0x0806
    }

    pub fn is_ipv6(&self) -> bool {
        self.ethertype == 0x86DD
    }
)}
