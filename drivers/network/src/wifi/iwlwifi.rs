//! Intel WiFi driver (iwlwifi equivalent)
//! Supports Intel AX200, AX201, AX210, AX211, BE200 (WiFi 6/6E/7)
//! This covers the majority of modern laptops.

/// Intel WiFi firmware metadata
pub struct IwlFirmware {
    pub name: &'static str,
    pub version: u32,
}

/// Supported Intel WiFi devices
pub const SUPPORTED_DEVICES: &[(u16, &str)] = &[
    (0x2723, "Intel Wi-Fi 6 AX200"),
    (0x02F0, "Intel Wi-Fi 6 AX201"),
    (0x2725, "Intel Wi-Fi 6E AX210"),
    (0x51F0, "Intel Wi-Fi 6E AX211"),
    (0x272B, "Intel Wi-Fi 7 BE200"),
];
