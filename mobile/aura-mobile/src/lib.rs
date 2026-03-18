//! Aura Mobile — Phone Companion App
//! 
//! A mini-AuraOS that runs on your phone.
//! Same Companion AI, same design language, seamless desktop connection.
//! 
//! Features:
//! - Aura Companion (same AI, synced context with desktop)
//! - Desktop notification mirror
//! - SMS/Call relay to desktop
//! - Clipboard sync
//! - File transfer (drag between phone and desktop)
//! - Phone as webcam/scanner/remote
//! - Find my phone (ring from desktop)
//! - Handoff (start on phone, continue on desktop)
//! - Phone as auth device (approve desktop logins)
//! - Quick actions (shortcuts to desktop commands)

pub mod bridge;
pub mod companion;
pub mod notifications;
pub mod auth;
pub mod discovery;
pub mod screens;

/// Connection state to the desktop
#[derive(Debug, Clone)]
pub struct DesktopConnection {
    pub state: ConnectionState,
    pub desktop_name: String,
    pub desktop_user: String,
    pub connection_type: MobileConnectionType,
    pub latency_ms: u32,
    pub paired_at: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Discovering,
    Pairing,
    Connected,
    Syncing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MobileConnectionType {
    SameWifi,
    BluetoothLE,
    USB,
    Internet,   // Different networks, relay through server
}

/// Pairing flow for first-time connection
/// 1. Phone discovers desktop on same WiFi or Bluetooth
/// 2. Desktop shows QR code
/// 3. Phone scans QR code
/// 4. Both exchange encryption keys
/// 5. Desktop shows phone's name + avatar
/// 6. User approves on desktop
/// 7. Permanent pairing established (survives reboots)
#[derive(Debug, Clone)]
pub struct PairingFlow {
    pub step: PairingStep,
    pub desktop_id: Option<String>,
    pub verification_code: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PairingStep {
    Scanning,
    QRDetected,
    ExchangingKeys,
    ShowVerificationCode,
    WaitingApproval,
    Paired,
    Failed,
}

/// Phone screens in the Aura Mobile app
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MobileScreen {
    /// Main screen — Companion chat + quick actions
    Home,
    /// Desktop remote control
    Remote,
    /// Notification center (desktop + phone unified)
    Notifications,
    /// File browser (desktop files accessible)
    Files,
    /// Camera mode (scanning, webcam relay)
    Camera,
    /// Settings
    Settings,
    /// Pairing / connection manager
    Connection,
}

/// Quick actions available on the phone
#[derive(Debug, Clone)]
pub struct QuickAction {
    pub id: String,
    pub label: String,
    pub icon: String,
    pub action: QuickActionType,
}

#[derive(Debug, Clone)]
pub enum QuickActionType {
    /// Lock the desktop
    LockDesktop,
    /// Play/pause media
    MediaPlayPause,
    /// Skip track
    MediaNext,
    /// Toggle do-not-disturb on desktop
    ToggleDND,
    /// Take screenshot on desktop
    Screenshot,
    /// Start screen sharing
    ScreenShare,
    /// Custom Companion command
    CompanionCommand(String),
    /// Open a specific surface on desktop
    OpenSurface(String),
    /// Run a terminal command
    RunCommand(String),
}
