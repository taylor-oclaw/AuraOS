//! WiFi network scanning
//! Discovers available networks, signal strength, and security types.

/// Scan result for a single WiFi network
#[derive(Debug, Clone)]
pub struct ScanResult {
    pub ssid: Vec<u8>,
    pub bssid: [u8; 6],
    pub channel: u8,
    pub frequency_mhz: u16,
    pub signal_dbm: i8,
    pub security: super::WifiSecurityType,
    pub is_hidden: bool,
}

/// Sort scan results by signal strength (strongest first)
pub fn sort_by_signal(results: &mut [ScanResult]) {
    results.sort_by(|a, b| b.signal_dbm.cmp(&a.signal_dbm));
}
