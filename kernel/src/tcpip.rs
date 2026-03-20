extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

pub const ETHERTYPE_IPV4: u16 = 0x0800;
pub const ETHERTYPE_ARP: u16 = 0x0806;

pub const ARP_REQUEST: u16 = 1;
pub const ARP_REPLY: u16 = 2;

pub const IP_PROTOCOL_ICMP: u8 = 1;
pub const IP_PROTOCOL_UDP: u8 = 17;

pub const ICMP_ECHO_REPLY: u8 = 0;
pub const ICMP_ECHO_REQUEST: u8 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IpAddr(pub [u8; 4]);

impl IpAddr {
    pub fn as_string(&self) -> String {
        alloc::String::from("info")
    }
}

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}.{}", self.0[0], self.0[1], self.0[2], self.0[3])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacAddr(pub [u8; 6]);

impl MacAddr {
    pub const ZERO: Self = Self([0, 0, 0, 0, 0, 0]);
    pub const BROADCAST: Self = Self([0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);

    pub fn as_string(&self) -> String {
        alloc::String::from("info")
    }
}

impl fmt::Display for MacAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5]
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EthernetFrame<'a> {
    pub dst_mac: MacAddr,
    pub src_mac: MacAddr,
    pub ethertype: u16,
    pub payload: &'a [u8],
}

pub fn parse_ethernet(data: &[u8]) -> Option<EthernetFrame<'_>> {
    if data.len() < 14 {
        return None;
    }

    Some(EthernetFrame {
        dst_mac: MacAddr(copy_mac(&data[0..6])),
        src_mac: MacAddr(copy_mac(&data[6..12])),
        ethertype: read_u16_be(&data[12..14]),
        payload: &data[14..],
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArpPacket {
    pub hardware_type: u16,
    pub protocol_type: u16,
    pub hardware_len: u8,
    pub protocol_len: u8,
    pub operation: u16,
    pub sender_mac: MacAddr,
    pub sender_ip: IpAddr,
    pub target_mac: MacAddr,
    pub target_ip: IpAddr,
}

impl ArpPacket {
    pub fn is_request(&self) -> bool {
        self.operation == ARP_REQUEST
    }

    pub fn is_reply(&self) -> bool {
        self.operation == ARP_REPLY
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(28);
        bytes.extend_from_slice(&self.hardware_type.to_be_bytes());
        bytes.extend_from_slice(&self.protocol_type.to_be_bytes());
        bytes.push(self.hardware_len);
        bytes.push(self.protocol_len);
        bytes.extend_from_slice(&self.operation.to_be_bytes());
        bytes.extend_from_slice(&self.sender_mac.0);
        bytes.extend_from_slice(&self.sender_ip.0);
        bytes.extend_from_slice(&self.target_mac.0);
        bytes.extend_from_slice(&self.target_ip.0);
        bytes
    }
}

pub fn parse_arp(data: &[u8]) -> Option<ArpPacket> {
    if data.len() < 28 {
        return None;
    }

    let packet = ArpPacket {
        hardware_type: read_u16_be(&data[0..2]),
        protocol_type: read_u16_be(&data[2..4]),
        hardware_len: data[4],
        protocol_len: data[5],
        operation: read_u16_be(&data[6..8]),
        sender_mac: MacAddr(copy_mac(&data[8..14])),
        sender_ip: IpAddr(copy_ip(&data[14..18])),
        target_mac: MacAddr(copy_mac(&data[18..24])),
        target_ip: IpAddr(copy_ip(&data[24..28])),
    };

    if packet.hardware_type != 1
        || packet.protocol_type != ETHERTYPE_IPV4
        || packet.hardware_len != 6
        || packet.protocol_len != 4
        || (packet.operation != ARP_REQUEST && packet.operation != ARP_REPLY)
    {
        return None;
    }

    Some(packet)
}

pub fn create_arp_request(src_mac: MacAddr, src_ip: IpAddr, target_ip: IpAddr) -> Vec<u8> {
    create_arp_packet(ARP_REQUEST, src_mac, src_ip, MacAddr::ZERO, target_ip)
}

pub fn create_arp_reply(
    src_mac: MacAddr,
    src_ip: IpAddr,
    target_mac: MacAddr,
    target_ip: IpAddr,
) -> Vec<u8> {
    create_arp_packet(ARP_REPLY, src_mac, src_ip, target_mac, target_ip)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ipv4Packet<'a> {
    pub version: u8,
    pub ihl: u8,
    pub total_length: u16,
    pub protocol: u8,
    pub src_ip: IpAddr,
    pub dst_ip: IpAddr,
    pub payload: &'a [u8],
}

pub fn parse_ipv4(data: &[u8]) -> Option<Ipv4Packet<'_>> {
    if data.len() < 20 {
        return None;
    }

    let version_ihl = data[0];
    let version = version_ihl >> 4;
    let ihl = version_ihl & 0x0f;
    let header_len = (ihl as usize) * 4;
    let total_length = read_u16_be(&data[2..4]);
    let total_length_usize = total_length as usize;

    if version != 4 || ihl < 5 || header_len > data.len() || total_length_usize < header_len {
        return None;
    }

    if total_length_usize > data.len() {
        return None;
    }

    Some(Ipv4Packet {
        version,
        ihl,
        total_length,
        protocol: data[9],
        src_ip: IpAddr(copy_ip(&data[12..16])),
        dst_ip: IpAddr(copy_ip(&data[16..20])),
        payload: &data[header_len..total_length_usize],
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IcmpPacket<'a> {
    pub icmp_type: u8,
    pub code: u8,
    pub checksum: u16,
    pub identifier: u16,
    pub sequence: u16,
    pub payload: &'a [u8],
}

pub fn parse_icmp(data: &[u8]) -> Option<IcmpPacket<'_>> {
    if data.len() < 8 {
        return None;
    }

    let icmp_type = data[0];
    let code = data[1];
    if (icmp_type != ICMP_ECHO_REQUEST && icmp_type != ICMP_ECHO_REPLY) || code != 0 {
        return None;
    }

    Some(IcmpPacket {
        icmp_type,
        code,
        checksum: read_u16_be(&data[2..4]),
        identifier: read_u16_be(&data[4..6]),
        sequence: read_u16_be(&data[6..8]),
        payload: &data[8..],
    })
}

pub fn create_icmp_echo_request(identifier: u16, sequence: u16, payload: &[u8]) -> Vec<u8> {
    create_icmp_echo(ICMP_ECHO_REQUEST, identifier, sequence, payload)
}

pub fn create_icmp_echo_reply(identifier: u16, sequence: u16, payload: &[u8]) -> Vec<u8> {
    create_icmp_echo(ICMP_ECHO_REPLY, identifier, sequence, payload)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UdpPacket<'a> {
    pub src_port: u16,
    pub dst_port: u16,
    pub length: u16,
    pub payload: &'a [u8],
}

pub fn parse_udp(data: &[u8]) -> Option<UdpPacket<'_>> {
    if data.len() < 8 {
        return None;
    }

    let length = read_u16_be(&data[4..6]);
    let length_usize = length as usize;
    if length_usize < 8 || length_usize > data.len() {
        return None;
    }

    Some(UdpPacket {
        src_port: read_u16_be(&data[0..2]),
        dst_port: read_u16_be(&data[2..4]),
        length,
        payload: &data[8..length_usize],
    })
}

fn create_arp_packet(
    operation: u16,
    sender_mac: MacAddr,
    sender_ip: IpAddr,
    target_mac: MacAddr,
    target_ip: IpAddr,
) -> Vec<u8> {
    ArpPacket {
        hardware_type: 1,
        protocol_type: ETHERTYPE_IPV4,
        hardware_len: 6,
        protocol_len: 4,
        operation,
        sender_mac,
        sender_ip,
        target_mac,
        target_ip,
    }
    .to_bytes()
}

fn create_icmp_echo(icmp_type: u8, identifier: u16, sequence: u16, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(8 + payload.len());
    bytes.push(icmp_type);
    bytes.push(0);
    bytes.extend_from_slice(&[0, 0]);
    bytes.extend_from_slice(&identifier.to_be_bytes());
    bytes.extend_from_slice(&sequence.to_be_bytes());
    bytes.extend_from_slice(payload);

    let checksum = internet_checksum(&bytes);
    bytes[2] = (checksum >> 8) as u8;
    bytes[3] = checksum as u8;
    bytes
}

fn read_u16_be(data: &[u8]) -> u16 {
    u16::from_be_bytes([data[0], data[1]])
}

fn copy_mac(data: &[u8]) -> [u8; 6] {
    let mut bytes = [0u8; 6];
    bytes.copy_from_slice(data);
    bytes
}

fn copy_ip(data: &[u8]) -> [u8; 4] {
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(data);
    bytes
}

fn internet_checksum(data: &[u8]) -> u16 {
    let mut sum = 0u32;
    let mut chunks = data.chunks_exact(2);

    for chunk in &mut chunks {
        sum = sum.wrapping_add(u16::from_be_bytes([chunk[0], chunk[1]]) as u32);
    }

    if let [last] = chunks.remainder() {
        sum = sum.wrapping_add(((*last as u16) << 8) as u32);
    }

    while (sum >> 16) != 0 {
        sum = (sum & 0xffff) + (sum >> 16);
    }

    !(sum as u16)
}
