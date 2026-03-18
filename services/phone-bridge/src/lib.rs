//! Aura Bridge — Seamless phone integration
//! 
//! Connects AuraOS desktop to your phone (Android & iOS).
//! The phone becomes an extension of the OS, not a separate device.
//! 
//! Connection methods:
//! - Same WiFi (local network discovery)
//! - Bluetooth LE (proximity pairing)
//! - USB (wired fallback)
//! - Internet relay (when on different networks)

pub mod discovery;
pub mod sync;
pub mod calls;
pub mod sms;

/// Connected phone information
#[derive(Debug, Clone)]
pub struct ConnectedPhone {
    pub id: String,
    pub name: String,
    pub platform: PhonePlatform,
    pub model: String,
    pub battery_percent: u8,
    pub signal_strength: SignalStrength,
    pub connection: ConnectionType,
    pub capabilities: PhoneCapabilities,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhonePlatform {
    Android,
    iOS,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    WiFiDirect,
    Bluetooth,
    USB,
    InternetRelay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalStrength {
    Excellent,
    Good,
    Fair,
    Poor,
    NoSignal,
}

#[derive(Debug, Clone, Copy)]
pub struct PhoneCapabilities {
    pub sms: bool,
    pub calls: bool,
    pub camera_front: bool,
    pub camera_rear: bool,
    pub gps: bool,
    pub nfc: bool,
    pub bluetooth: bool,
    pub clipboard: bool,
    pub file_transfer: bool,
    pub notifications: bool,
    pub hotspot: bool,
    pub find_my_phone: bool,
}

/// Things you can do with the phone
#[derive(Debug, Clone)]
pub enum PhoneAction {
    // Communication
    SendSMS { to: String, message: String },
    MakeCall { to: String },
    AnswerCall,
    HangUp,
    
    // Continuity
    ClipboardSync { content: ClipboardContent },
    HandoffUrl { url: String },
    HandoffFile { path: String },
    
    // Phone as sensor
    UseAsWebcam { camera: PhoneCamera },
    UseAsScanner,
    UseAsRemote,
    
    // Utility
    FindMyPhone { ring: bool },
    EnableHotspot,
    FileTransfer { files: Vec<String>, direction: TransferDirection },
    
    // Notifications
    MirrorNotifications { enabled: bool },
    DismissNotification { id: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhoneCamera { Front, Rear }

#[derive(Debug, Clone)]
pub enum ClipboardContent {
    Text(String),
    Image(Vec<u8>),
    Url(String),
    File(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferDirection { PhoneToDesktop, DesktopToPhone }
