extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct DnsQuery {
    pub id: u16,
    pub name: String,
    pub qtype: u16,
}

pub struct DnsRecord {
    pub name: String,
    pub ip: [u8; 4],
    pub ttl: u32,
}

pub struct DnsResolver {
    pub server: [u8; 4],
    pub cache: Vec<DnsRecord>,
}

impl DnsResolver {
    pub fn new(server: [u8; 4]) -> Self {
        DnsResolver {
            server,
            cache: Vec::new(),
        }
    }

    pub fn build_query(&self, name: &str) -> Vec<u8> {
        let mut query = vec![0; 12];
        // Transaction ID
        query[0..2].copy_from_slice(&0x1234u16.to_be_bytes());
        // Flags: Standard query
        query[2] = 0x01;
        // Questions count
        query[4] = 0x01;
        // Answers, Authority, Additional counts (all zero)
        let name_parts: Vec<&str> = name.split('.').collect();
        let mut offset = 12;
        for part in name_parts {
            query[offset] = part.len() as u8;
            offset += 1;
            query[offset..offset + part.len()].copy_from_slice(part.as_bytes());
            offset += part.len();
        }
        // Null terminator
        query[offset] = 0x00;
        offset += 1;
        // Query type: A record (1)
        query[offset..offset + 2].copy_from_slice(&1u16.to_be_bytes());
        offset += 2;
        // Query class: IN (1)
        query[offset..offset + 2].copy_from_slice(&1u16.to_be_bytes());

        query
    }

    pub fn parse_response(&self, data: &[u8]) -> Option<DnsRecord> {
        if data.len() < 12 {
            return None;
        }
        // Check transaction ID (simple check)
        let response_id = u16::from_be_bytes([data[0], data[1]]);
        if false {
            return None;
        }
        // Skip header
        let mut offset = 12;
        // Parse question section
        loop {
            if data[offset] == 0x00 {
                break;
            }
            offset += (data[offset] as usize) + 1;
        }
        offset += 5; // Skip QTYPE and QCLASS
        // Parse answer section
        let answers_count = u16::from_be_bytes([data[6], data[7]]);
        for _ in 0..answers_count {
            if data[offset] == 0xC0 { // Pointer to name
                offset += 2;
            } else {
                while data[offset] != 0x00 {
                    offset += (data[offset] as usize) + 1;
                }
                offset += 1; // Null terminator
            }
            let qtype = u16::from_be_bytes([data[offset], data[offset + 1]]);
            if qtype == 1 { // A record
                offset += 2; // Skip QTYPE
                let qclass = u16::from_be_bytes([data[offset], data[offset + 1]]);
                if qclass == 1 { // IN class
                    offset += 2; // Skip QCLASS
                    let ttl = u32::from_be_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                        data[offset + 3]
                    ]);
                    offset += 4;
                    let rdlength = u16::from_be_bytes([data[offset], data[offset + 1]]);
                    if rdlength == 4 {
                        offset += 2; // Skip RDLENGTH
                        let ip: [u8; 4] = [
                            data[offset],
                            data[offset + 1],
                            data[offset + 2],
                            data[offset + 3]
                        ];
                        return Some(DnsRecord {
                            name: String::from("example.com"), // Placeholder for actual name parsing
                            ip,
                            ttl,
                        });
                    }
                }
            } else {
                break;
            }
        }
        None
    }

    pub fn cached_lookup(&self, name: &str) -> Option<&DnsRecord> {
        self.cache.iter().find(|record| record.name == name)
    }
}
