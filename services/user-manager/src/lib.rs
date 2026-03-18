//! AuraOS Multi-User Manager
//! 
//! No passwords. Identity through presence.
//! 
//! The computer recognizes WHO you are through:
//! 1. Face (primary — 200ms recognition)
//! 2. Voice (secondary — "Hey Aura, it's me")
//! 3. Phone proximity (passive — Bluetooth beacon)
//! 4. Typing pattern (continuous — behavioral biometrics)
//! 5. Custom gesture (fun — secret handshake)
//!
//! Multi-user: Walk up → your desktop. Someone else walks up → their desktop.
//! No login screen. No password. No waiting.

pub mod profiles;
pub mod enrollment;
pub mod switching;
pub mod guest;
pub mod parental;

/// A user profile on this machine
#[derive(Debug, Clone)]
pub struct UserProfile {
    pub id: UserId,
    pub name: String,
    pub display_name: String,
    pub avatar: Option<Avatar>,
    pub role: UserRole,
    pub biometrics: EnrolledBiometrics,
    pub preferences: UserPreferences,
    pub created_at: u64,
    pub last_active_at: u64,
    pub login_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(pub u64);

#[derive(Debug, Clone)]
pub enum Avatar {
    /// Generated from user's face (privacy-respecting abstract art)
    GeneratedFromFace,
    /// Custom image
    Image(Vec<u8>),
    /// Emoji
    Emoji(String),
    /// Initials with color
    Initials { text: String, color: (u8, u8, u8) },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserRole {
    /// Full access, can manage other users
    Owner,
    /// Full access to their own space
    Adult,
    /// Restricted access, parental controls
    Child,
    /// Temporary access, no personal data stored
    Guest,
    /// Managed by enterprise policy
    Managed,
}

/// What biometric methods this user has enrolled
#[derive(Debug, Clone)]
pub struct EnrolledBiometrics {
    pub face: bool,
    pub face_angles: u8,         // How many angles enrolled (more = better)
    pub voice: bool,
    pub voice_phrases: u8,       // How many phrases enrolled
    pub phone_paired: bool,
    pub phone_name: Option<String>,
    pub typing_pattern: bool,
    pub custom_gesture: bool,
    pub pin_set: bool,           // Fallback PIN
    pub recovery_key: bool,      // Emergency recovery
}

/// Per-user preferences (loaded on recognition)
#[derive(Debug, Clone)]
pub struct UserPreferences {
    pub theme: String,
    pub language: String,
    pub timezone: String,
    pub companion_name: String,       // What they call their AI companion
    pub companion_voice: String,      // Which voice the companion uses
    pub companion_personality: String, // Formal, casual, playful, etc.
    pub dominant_hand: Hand,
    pub text_size: TextSize,
    pub reduce_motion: bool,
    pub high_contrast: bool,
    pub notification_level: NotificationLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hand { Left, Right }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextSize { Small, Medium, Large, ExtraLarge }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationLevel {
    All,
    Important,
    Urgent,
    None,
}

/// What happens when a user is recognized
#[derive(Debug, Clone)]
pub enum RecognitionAction {
    /// Known user — switch to their session
    SwitchToUser { user_id: UserId, method: AuthMethod },
    /// Unknown face — offer guest mode or enrollment
    UnknownPerson { offer_guest: bool, offer_enroll: bool },
    /// Known child — switch with parental restrictions
    ChildDetected { user_id: UserId, restrictions: Vec<String> },
    /// No face detected — show ambient display
    NobodyHome,
    /// Multiple faces — ask who wants to use it
    MultiplePeople { recognized: Vec<UserId>, unknown_count: u8 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthMethod {
    Face,
    Voice,
    PhoneProximity,
    TypingPattern,
    CustomGesture,
    PinFallback,
    RecoveryKey,
}

/// Ambient display shown when nobody is using the computer
/// (instead of a lock screen)
#[derive(Debug, Clone)]
pub struct AmbientDisplay {
    pub mode: AmbientMode,
    pub show_time: bool,
    pub show_weather: bool,
    pub show_upcoming: bool,     // Next calendar event
    pub show_photos: bool,       // Photo slideshow
    pub show_music_art: bool,    // Currently playing album art
    pub dim_after_minutes: u32,
    pub sleep_after_minutes: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmbientMode {
    /// Clock + weather + subtle info
    InfoDisplay,
    /// Photo slideshow
    PhotoFrame,
    /// Abstract art / screensaver
    ArtMode,
    /// Completely dark (OLED power save)
    Dark,
    /// Custom — user-defined surface
    Custom,
}
