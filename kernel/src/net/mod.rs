//! AuraOS Network Subsystem
//! 
//! Supports both wired (Ethernet) and wireless (WiFi) during setup and runtime.
//! The network stack follows the microkernel philosophy: minimal kernel support,
//! full implementation in userspace drivers.

pub mod phy;      // Physical layer abstraction
pub mod buffer;   // Network buffer management

/// Network device capability flags
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct NetCapabilities: u32 {
        const ETHERNET   = 0b0000_0001;
        const WIFI       = 0b0000_0010;
        const BLUETOOTH  = 0b0000_0100;
        const CELLULAR   = 0b0000_1000;
        const CHECKSUM   = 0b0001_0000;
        const TSO        = 0b0010_0000; // TCP segmentation offload
        const SCATTER    = 0b0100_0000; // Scatter-gather DMA
        const WOL        = 0b1000_0000; // Wake-on-LAN
    }
}

/// Represents a discovered network interface
#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub id: u32,
    pub name: [u8; 16],       // e.g., "eth0", "wlan0"
    pub mac: [u8; 6],
    pub capabilities: NetCapabilities,
    pub link_up: bool,
    pub speed_mbps: u32,      // 0 = unknown
    pub driver: DriverKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverKind {
    /// Intel e1000/e1000e family (most VMs + common hardware)
    IntelE1000,
    /// Virtio-net (QEMU/KVM)
    VirtioNet,
    /// Realtek RTL8139/RTL8169 family
    Realtek,
    /// Intel WiFi (iwlwifi equivalent)
    IntelWifi,
    /// Broadcom WiFi
    BroadcomWifi,
    /// Qualcomm Atheros WiFi
    AtherosWifi,
    /// MediaTek WiFi
    MediaTekWifi,
    /// Generic/unknown
    Generic,
}

/// WiFi security types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WifiSecurity {
    Open,
    WEP,
    WPA,
    WPA2Personal,
    WPA2Enterprise,
    WPA3Personal,
    WPA3Enterprise,
}

/// A discovered WiFi network
#[derive(Debug, Clone)]
pub struct WifiNetwork {
    pub ssid: [u8; 32],
    pub ssid_len: usize,
    pub bssid: [u8; 6],
    pub channel: u8,
    pub signal_dbm: i8,        // Signal strength in dBm
    pub signal_percent: u8,    // 0-100
    pub security: WifiSecurity,
    pub frequency_mhz: u16,
}

/// Network configuration obtained during setup
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub method: ConfigMethod,
    pub ipv4_addr: [u8; 4],
    pub ipv4_mask: [u8; 4],
    pub ipv4_gateway: [u8; 4],
    pub dns_primary: [u8; 4],
    pub dns_secondary: [u8; 4],
    pub hostname: [u8; 64],
    pub hostname_len: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigMethod {
    Dhcp,
    Static,
    LinkLocal,
}
