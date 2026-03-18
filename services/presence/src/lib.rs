//! Aura Presence — Knows when you're there
//! 
//! Fuses camera, microphone, keyboard/mouse, and phone proximity
//! to determine if the user is present, away, or someone else is there.
//! 
//! This drives:
//! - Screen wake/sleep
//! - Face unlock
//! - Privacy mode (someone behind you)
//! - Companion behavior (quiet when you're away)
//! - Power management

/// Current presence state
#[derive(Debug, Clone)]
pub struct PresenceState {
    pub user: UserPresence,
    pub privacy: PrivacyLevel,
    pub since: u64,              // Timestamp of last state change
    pub confidence: f32,
    pub sensors_contributing: Vec<PresenceSensor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserPresence {
    /// User is at the computer, engaged
    Active,
    /// User is nearby but not actively using (e.g., reading phone)
    Idle,
    /// User stepped away briefly
    BrieflyAway,
    /// User has been gone for a while
    Away,
    /// User is sleeping (late night + no activity)
    Sleeping,
    /// Cannot determine (sensors unavailable)
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivacyLevel {
    /// User is alone — full access
    Private,
    /// Known person nearby (family member)
    Trusted,
    /// Unknown person detected — blur sensitive content
    Guarded,
    /// Multiple people — minimize personal info
    Public,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresenceSensor {
    Camera,         // Face detection
    Microphone,     // Voice activity
    Keyboard,       // Keystrokes
    Mouse,          // Movement
    PhoneProximity, // Phone Bluetooth RSSI
    TouchScreen,    // Touch events
}

/// Actions the OS takes based on presence
#[derive(Debug, Clone)]
pub enum PresenceAction {
    WakeScreen,
    DimScreen,
    LockScreen,
    UnlockWithFace,
    EnablePrivacyMode,
    DisablePrivacyMode,
    MuteNotificationSounds,
    UnmuteNotificationSounds,
    PauseMedia,
    ResumeMedia,
    AdjustBrightness(f32),
    CompanionGreeting(String),
}
