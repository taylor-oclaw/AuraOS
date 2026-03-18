//! AuraOS Network Service
//! Full TCP/IP stack running in userspace.
//! Provides: Ethernet frames → IP → TCP/UDP → DNS → HTTP

pub mod ip;
pub mod tcp;
pub mod udp;
pub mod dns;
pub mod arp;
