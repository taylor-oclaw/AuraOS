//! Physical network device abstraction
//! Kernel only provides PCI device discovery and MMIO mapping.
//! Actual driver logic runs in userspace.

/// PCI network device information passed to userspace drivers
#[derive(Debug, Clone, Copy)]
pub struct PciNetDevice {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class: u8,
    pub subclass: u8,
    pub bar0: u64,
    pub bar1: u64,
    pub irq: u8,
}

/// Known PCI vendor/device IDs for network hardware
pub mod known_devices {
    // Intel Ethernet
    pub const INTEL_VENDOR: u16 = 0x8086;
    pub const E1000_82540EM: u16 = 0x100E;  // QEMU default
    pub const E1000E_82574L: u16 = 0x10D3;
    pub const I210: u16 = 0x1533;
    pub const I225: u16 = 0x15F3;
    
    // Intel WiFi
    pub const WIFI_AX200: u16 = 0x2723;    // WiFi 6
    pub const WIFI_AX201: u16 = 0x02F0;
    pub const WIFI_AX210: u16 = 0x2725;
    pub const WIFI_AX211: u16 = 0x51F0;
    pub const WIFI_BE200: u16 = 0x272B;    // WiFi 7
    
    // Realtek
    pub const REALTEK_VENDOR: u16 = 0x10EC;
    pub const RTL8139: u16 = 0x8139;
    pub const RTL8169: u16 = 0x8169;
    pub const RTL8125: u16 = 0x8125;      // 2.5GbE
    
    // Virtio (QEMU/KVM)
    pub const VIRTIO_VENDOR: u16 = 0x1AF4;
    pub const VIRTIO_NET: u16 = 0x1000;
    pub const VIRTIO_NET_MODERN: u16 = 0x1041;
    
    // Broadcom WiFi
    pub const BROADCOM_VENDOR: u16 = 0x14E4;
    
    // Qualcomm/Atheros WiFi
    pub const QUALCOMM_VENDOR: u16 = 0x168C;
    
    // MediaTek WiFi
    pub const MEDIATEK_VENDOR: u16 = 0x14C3;
}
