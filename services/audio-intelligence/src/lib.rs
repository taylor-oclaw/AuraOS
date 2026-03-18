//! Aura Audio Intelligence — smart hearing for the OS
//! 
//! - Wake word detection ("Hey Aura")
//! - Continuous speech-to-text (Whisper-based)
//! - Sound classification (doorbell, alarm, baby, dog, music, speech)
//! - Voice biometrics (who is speaking?)
//! - Meeting transcription + summarization
//! - Ambient noise monitoring

/// Detected sound event
#[derive(Debug, Clone)]
pub struct SoundEvent {
    pub timestamp_ns: u64,
    pub kind: SoundKind,
    pub confidence: f32,
    pub direction_deg: Option<f32>,  // From mic array
    pub duration_ms: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundKind {
    Speech,
    Music,
    Doorbell,
    Knock,
    Alarm,
    BabyCry,
    DogBark,
    CatMeow,
    PhoneRing,
    Appliance,
    Siren,
    Thunder,
    Silence,
    Unknown,
}

/// Voice identity result
#[derive(Debug, Clone)]
pub struct VoiceIdentity {
    pub user_id: Option<String>,
    pub name: Option<String>,
    pub confidence: f32,
    pub is_registered: bool,
}

/// Transcription segment
#[derive(Debug, Clone)]
pub struct TranscriptSegment {
    pub text: String,
    pub start_ms: u64,
    pub end_ms: u64,
    pub speaker: Option<VoiceIdentity>,
    pub confidence: f32,
    pub language: String,
    pub is_final: bool,
}

/// Wake word detection result
#[derive(Debug, Clone)]
pub struct WakeWordEvent {
    pub word: String,      // "Hey Aura" or custom
    pub confidence: f32,
    pub speaker: Option<VoiceIdentity>,
}
