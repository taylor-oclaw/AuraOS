//! AuraOS Session Manager
//! 
//! Manages user sessions with INSTANT switching.
//! When User A walks away and User B walks up:
//! 1. User A's session is suspended (surfaces frozen in memory)
//! 2. User B's session is restored (surfaces reappear exactly where they were)
//! 3. Total switch time: < 500ms
//! 
//! Each session is completely isolated:
//! - Separate file encryption keys
//! - Separate clipboard
//! - Separate network sessions
//! - Separate Companion context

/// A user session — everything about their current state
#[derive(Debug)]
pub struct UserSession {
    pub user_id: u64,
    pub state: SessionState,
    pub started_at: u64,
    pub last_active_at: u64,
    
    /// Encrypted session key (derived from biometric + hardware key)
    pub session_key: [u8; 32],
    
    /// Active surfaces (frozen when suspended)
    pub surfaces: Vec<FrozenSurface>,
    
    /// Companion conversation context
    pub companion_context: Vec<u8>,
    
    /// Clipboard contents (encrypted, per-user)
    pub clipboard: Option<Vec<u8>>,
    
    /// Active network connections
    pub network_sessions: Vec<NetworkSession>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    /// Currently displayed — user is active
    Active,
    /// In memory but not displayed — user walked away
    Suspended,
    /// Saved to disk — freed memory for active users
    Hibernated,
    /// Being created (first-time or fresh session)
    Initializing,
    /// Being torn down
    Terminating,
}

/// A surface saved in suspended state
#[derive(Debug)]
pub struct FrozenSurface {
    pub surface_id: u64,
    pub surface_type: String,
    pub position: (f32, f32, f32, f32), // x, y, w, h
    pub z_order: i32,
    /// Framebuffer snapshot for instant visual restore
    pub thumbnail: Option<Vec<u8>>,
    /// Full state for app restore
    pub state_blob: Vec<u8>,
}

#[derive(Debug)]
pub struct NetworkSession {
    pub kind: String,     // "http", "websocket", "ssh", etc.
    pub remote: String,
    pub encrypted: bool,
}

/// Session switch animation
/// Shows a brief, elegant transition between users
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchAnimation {
    /// Surfaces dissolve out, new ones materialize in
    Dissolve,
    /// Current session slides away, new slides in
    Slide,
    /// Crossfade between the two
    Crossfade,
    /// Instant (for speed-critical situations)
    Instant,
}
