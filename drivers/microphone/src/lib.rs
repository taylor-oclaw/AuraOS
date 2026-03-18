//! AuraOS Microphone Driver
//! The microphone is a sense organ, not just an input device.

pub mod pipeline;

/// Audio input device
#[derive(Debug, Clone)]
pub struct AudioInputDevice {
    pub id: u32,
    pub name: String,
    pub kind: MicrophoneKind,
    pub channels: u8,
    pub sample_rates: Vec<u32>,
    pub bit_depth: u8,
    pub connected: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MicrophoneKind {
    BuiltIn,         // Laptop built-in mic
    USB,             // USB microphone
    Bluetooth,       // BT headset mic
    Array,           // Multi-mic array (spatial audio input)
    PhoneMic,        // Phone microphone (via Bridge)
    Virtual,         // Software-generated audio
}

/// Audio frame
#[derive(Debug)]
pub struct AudioFrame {
    pub device_id: u32,
    pub sample_rate: u32,
    pub channels: u8,
    pub samples: Vec<f32>,    // Interleaved float samples
    pub timestamp_ns: u64,
}

/// Audio processing pipeline stages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioStage {
    /// Raw capture from hardware
    RawCapture,
    /// Echo cancellation (remove speaker output from mic input)
    EchoCancellation,
    /// Noise suppression (remove background noise)
    NoiseSuppression,
    /// Automatic gain control
    GainControl,
    /// Voice activity detection
    VoiceDetection,
    /// Wake word detection
    WakeWordDetection,
    /// Speech-to-text ready
    SpeechReady,
}
