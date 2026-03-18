//! Virtio-net driver for QEMU/KVM virtualization
//! High performance paravirtualized networking.

/// Virtio-net device configuration
#[repr(C)]
pub struct VirtioNetConfig {
    pub mac: [u8; 6],
    pub status: u16,
    pub max_virtqueue_pairs: u16,
    pub mtu: u16,
}
