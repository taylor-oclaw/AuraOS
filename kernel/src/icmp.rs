extern crate alloc;
use alloc::vec::Vec;
use alloc::vec;

pub struct IcmpPacket {
    pub icmp_type: u8,
    pub code: u8,
    pub checksum: u16,
    pub id: u16,
    pub seq: u16,
    pub data: Vec<u8>,
}

impl IcmpPacket {
    pub fn echo_request(id: u16, seq: u16, data: &[u8]) -> Self {
        Self {
            icmp_type: 8,
            code: 0,
            checksum: 0,
            id,
            seq,
            data: data.to_vec(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut pkt = vec![self.icmp_type, self.code];
        pkt.extend_from_slice(&self.checksum.to_be_bytes());
        pkt.extend_from_slice(&self.id.to_be_bytes());
        pkt.extend_from_slice(&self.seq.to_be_bytes());
        pkt.extend_from_slice(&self.data);
        let cs = checksum(&pkt);
        pkt[2..4].copy_from_slice(&cs.to_be_bytes());
        pkt
    }

    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 8 {
            return None;
        }
        Some(Self {
            icmp_type: data[0],
            code: data[1],
            checksum: u16::from_be_bytes([data[2], data[3]]),
            id: u16::from_be_bytes([data[4], data[5]]),
            seq: u16::from_be_bytes([data[6], data[7]]),
            data: data[8..].to_vec(),
        })
    }

    pub fn is_echo_reply(&self) -> bool {
        self.icmp_type == 0 && self.code == 0
    }
}

fn checksum(data: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    for i in (0..data.len()).step_by(2) {
        let word = if i + 1 < data.len() {
            ((data[i] as u32) << 8) | data[i + 1] as u32
        } else {
            (data[i] as u32) << 8
        };
        sum += word;
    }
    while sum >> 16 != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    !(sum as u16)
}
