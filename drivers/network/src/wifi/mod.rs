//! WiFi driver framework
//! Handles scanning, authentication, and association.

pub mod scan;
pub mod wpa;
pub mod iwlwifi;

/// WiFi driver state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WifiState {
    Disconnected,
    Scanning,
    Authenticating,
    Associating,
    Connected,
    Disconnecting,
    Error,
}

/// WiFi connection request
#[derive(Debug, Clone)]
pub struct ConnectRequest {
    pub ssid: Vec<u8>,
    pub password: Option<Vec<u8>>,
    pub security: WifiSecurityType,
    pub hidden: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WifiSecurityType {
    Open,
    WPA2,
    WPA3,
}
