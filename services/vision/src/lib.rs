//! Aura Vision — AI-powered visual intelligence
//! 
//! Processes camera frames for:
//! - Face detection & recognition (unlock, multi-user)
//! - Gesture recognition (hand tracking, body poses)
//! - Object detection (document scanning, product lookup)
//! - Scene understanding (is user at desk? in kitchen? outdoors?)
//! - Emotion detection (opt-in, for companion awareness)
//! - Person counting (privacy mode when others present)
//! - Background segmentation (blur/replace in video calls)

pub mod face;
pub mod gesture;
pub mod scene;
pub mod background;

/// Vision processing result for a single frame
#[derive(Debug, Clone)]
pub struct VisionFrame {
    pub timestamp_ns: u64,
    pub faces: Vec<DetectedFace>,
    pub hands: Vec<DetectedHand>,
    pub gestures: Vec<RecognizedGesture>,
    pub scene: SceneContext,
    pub people_count: u8,
    pub primary_user_present: bool,
    pub attention: AttentionState,
}

/// A detected face in the frame
#[derive(Debug, Clone)]
pub struct DetectedFace {
    pub id: u32,
    pub bounds: (f32, f32, f32, f32), // x, y, width, height (normalized 0-1)
    pub confidence: f32,
    pub identity: Option<FaceIdentity>,
    pub emotion: Option<Emotion>,
    pub gaze_direction: Option<(f32, f32)>, // x, y angles
    pub eyes_open: bool,
    pub mouth_open: bool,
}

#[derive(Debug, Clone)]
pub struct FaceIdentity {
    pub user_id: String,
    pub name: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Emotion {
    Neutral,
    Happy,
    Sad,
    Surprised,
    Confused,
    Frustrated,
    Focused,
    Tired,
}

/// A detected hand
#[derive(Debug, Clone)]
pub struct DetectedHand {
    pub side: HandSide,
    pub landmarks: Vec<(f32, f32, f32)>, // 21 hand landmarks (x, y, z)
    pub gesture: Option<HandGesture>,
    pub confidence: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandSide { Left, Right }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandGesture {
    Open,           // Open palm — pause/stop
    Fist,           // Closed fist
    PointUp,        // Index finger up — select
    PointRight,     // Point right — next
    PointLeft,      // Point left — previous
    ThumbsUp,       // Approval — confirm action
    ThumbsDown,     // Disapproval — cancel
    Peace,          // ✌️ — take screenshot
    Pinch,          // Thumb + index — zoom
    Wave,           // Waving — dismiss notification
    SwipeLeft,      // Hand swipe left
    SwipeRight,     // Hand swipe right
}

/// Recognized gesture as an OS action
#[derive(Debug, Clone)]
pub struct RecognizedGesture {
    pub gesture: HandGesture,
    pub action: GestureAction,
    pub confidence: f32,
}

#[derive(Debug, Clone)]
pub enum GestureAction {
    DismissNotification,
    PauseMedia,
    PlayMedia,
    NextTrack,
    PreviousTrack,
    VolumeUp,
    VolumeDown,
    Screenshot,
    Confirm,
    Cancel,
    ScrollUp,
    ScrollDown,
    Custom(String),
}

/// Scene understanding
#[derive(Debug, Clone)]
pub struct SceneContext {
    pub environment: Environment,
    pub lighting: Lighting,
    pub objects: Vec<DetectedObject>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Desk,
    Kitchen,
    LivingRoom,
    Bedroom,
    Outdoors,
    Car,
    Office,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub struct Lighting {
    pub brightness: f32,     // 0-1 (dark to bright)
    pub color_temp_k: u32,   // Color temperature
    pub natural_light: bool, // Window/sunlight detected
}

#[derive(Debug, Clone)]
pub struct DetectedObject {
    pub label: String,
    pub confidence: f32,
    pub bounds: (f32, f32, f32, f32),
}

/// User attention state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttentionState {
    /// User is looking at the screen
    Focused,
    /// User is present but looking away
    Distracted,
    /// User stepped away recently (< 2 min)
    Away,
    /// User has been gone > 2 min
    Gone,
    /// Multiple people detected — privacy mode
    SharedViewing,
}
