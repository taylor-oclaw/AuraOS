extern crate alloc;
use alloc::vec::Vec;
use alloc::vec;

pub struct ArpEntry {
    pub ip: [u8; 4],
    pub mac: [u8; 6],
    pub ttl: u32,
}

pub struct ArpTable {
    pub entries: Vec<ArpEntry>,
}

impl ArpTable {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn lookup(&self, ip: &[u8; 4]) -> Option<[u8; 6]> {
        for e in &self.entries {
            if &e.ip == ip {
                return Some(e.mac);
            }
        }
        None
    }

    pub fn insert(&mut self, ip: [u8; 4], mac: [u8; 6], ttl: u32) {
        for e in self.entries.iter_mut() {
            if e.ip == ip {
                e.mac = mac;
                e.ttl = ttl;
                return;
            }
        }
        self.entries.push(ArpEntry { ip, mac, ttl });
    }

    pub fn build_request(sender_ip: [u8; 4], sender_mac: [u8; 6], target_ip: [u8; 4]) -> Vec<u8> {
        let mut pkt = vec![0u8; 28];
        pkt[0..2].copy_from_slice(&[0x00, 0x01]);
        pkt[2..4].copy_from_slice(&[0x08, 0x00]);
        pkt[4] = 6;
        pkt[5] = 4;
        pkt[6..8].copy_from_slice(&[0x00, 0x01]);
        pkt[8..14].copy_from_slice(&sender_mac);
        pkt[14..18].copy_from_slice(&sender_ip);
        pkt[22..28].fill(0);
        pkt[24..28].copy_from_slice(&target_ip);
        pkt
    }

    pub fn tick(&mut self) {
        self.entries.retain(|e| e.ttl > 0);
        for e in self.entries.iter_mut() {
            e.ttl -= 1;
        }
    }
}
