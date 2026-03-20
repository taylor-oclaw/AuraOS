extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SerialPort {
    pub port_base: u16,
    pub baud_rate: u32,
    pub initialized: bool,
    pub tx_buffer: Vec<u8>,
    pub rx_buffer: Vec<u8>,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

impl SerialPort {
    pub fn new(base: u16, baud: u32) -> Self {
        Self { port_base: base, baud_rate: baud, initialized: false, tx_buffer: Vec::new(), rx_buffer: Vec::new(), bytes_sent: 0, bytes_received: 0 }
    }
    pub fn init(&mut self) { self.initialized = true; }
    pub fn write_byte(&mut self, b: u8) { if self.initialized { self.tx_buffer.push(b); self.bytes_sent += 1; } }
    pub fn write_str(&mut self, s: &str) { for b in s.bytes() { self.write_byte(b); } }
    pub fn read_byte(&mut self) -> Option<u8> { if self.rx_buffer.is_empty() { None } else { self.bytes_received += 1; Some(self.rx_buffer.remove(0)) } }
    pub fn flush(&mut self) { self.tx_buffer.clear(); }
    pub fn is_ready(&self) -> bool { self.initialized }
}
