//! Network packet buffer management
//! Pre-allocated ring buffers for zero-copy networking.

const MAX_PACKET_SIZE: usize = 9216; // Jumbo frame support
const RX_RING_SIZE: usize = 256;
const TX_RING_SIZE: usize = 256;

#[repr(C, align(4096))]
pub struct PacketBuffer {
    pub data: [u8; MAX_PACKET_SIZE],
    pub len: usize,
    pub flags: u32,
}

/// Ring buffer for network packet DMA
pub struct PacketRing {
    buffers: &'static mut [PacketBuffer],
    head: usize,
    tail: usize,
    capacity: usize,
}

impl PacketRing {
    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }
    
    pub fn is_full(&self) -> bool {
        (self.tail + 1) % self.capacity == self.head
    }
}
