extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum VoiceState {
    Idle,
    Listening,
    Processing,
    Speaking,
    Error(String),
}

pub struct VoiceCommand {
    pub text: String,
    pub confidence: f32,
    pub language: String,
    pub agent_target: Option<u64>,
}

pub struct VoiceResponse {
    pub text: String,
    pub audio_data: Vec<u8>,
    pub voice_id: String,
    pub duration_ms: u64,
}

pub struct VoicePipeline {
    pub state: VoiceState,
    pub commands: Vec<VoiceCommand>,
    pub responses: Vec<VoiceResponse>,
    pub wake_word: String,
    pub active_voice: String,
    pub sample_rate: u32,
    pub channels: u8,
    pub noise_threshold: f32,
}

impl VoicePipeline {
    pub fn new() -> Self {
        Self {
            state: VoiceState::Idle,
            commands: Vec::new(),
            responses: Vec::new(),
            wake_word: String::from("aura"),
            active_voice: String::from("default"),
            sample_rate: 16000,
            channels: 1,
            noise_threshold: 0.3,
        }
    }

    pub fn start_listening(&mut self) {
        self.state = VoiceState::Listening;
    }

    pub fn process_command(&mut self, text: &str, confidence: f32, lang: &str) {
        self.commands.push(VoiceCommand {
            text: String::from(text),
            confidence,
            language: String::from(lang),
            agent_target: None,
        });
        self.state = VoiceState::Processing;
    }

    pub fn speak(&mut self, text: &str, voice: &str) {
        self.responses.push(VoiceResponse {
            text: String::from(text),
            audio_data: Vec::new(),
            voice_id: String::from(voice),
            duration_ms: 0,
        });
        self.state = VoiceState::Speaking;
    }

    pub fn set_wake_word(&mut self, word: &str) {
        self.wake_word = String::from(word);
    }

    pub fn is_listening(&self) -> bool {
        matches!(self.state, VoiceState::Listening)
    }

    pub fn last_command(&self) -> Option<&VoiceCommand> {
        self.commands.last()
    }

    pub fn command_count(&self) -> usize {
        self.commands.len()
    }
}
