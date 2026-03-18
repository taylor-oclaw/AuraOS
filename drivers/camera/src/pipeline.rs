//! Camera pipeline — routes frames to consumers
//! Multiple services can subscribe to camera frames simultaneously:
//! - Vision service (face detection, gesture recognition)
//! - Presence service (person detection)
//! - Video calls (stream to remote)
//! - Background blur (process and forward)

/// Frame consumer registration
pub struct FrameConsumer {
    pub id: String,
    pub priority: ConsumerPriority,
    pub resolution: Option<(u32, u32)>,  // Desired resolution (downscale for perf)
    pub fps: Option<u32>,                 // Desired FPS
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConsumerPriority {
    /// Security/biometric (face unlock) — never drops frames
    Critical = 0,
    /// Real-time (video calls) — low latency required
    RealTime = 1,
    /// Interactive (gesture control) — some frame drops ok
    Interactive = 2,
    /// Background (presence detection) — can run at low FPS
    Background = 3,
    /// Batch (document scanning) — processes when available
    Batch = 4,
}

/// Privacy indicator — ALWAYS shown when camera is active
/// Cannot be hidden or overridden by any application.
#[derive(Debug, Clone, Copy)]
pub struct PrivacyIndicator {
    /// Green dot in system bar when camera is active
    pub camera_active: bool,
    /// Orange dot when microphone is active  
    pub microphone_active: bool,
    /// Which consumers are using the camera
    pub active_consumers: u32,
}
