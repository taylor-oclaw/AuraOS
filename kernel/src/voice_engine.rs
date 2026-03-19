extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AudioSample {
    pub data: Vec<i16>,
    pub sample_rate: u32,
    pub channels: u8,
}

pub enum VoiceCommand {
    Text(String),
    SystemControl(SystemAction),
    AppLaunch(String),
    FileOperation(String),
    Unknown,
}

pub enum SystemAction {
    VolumeUp,
    VolumeDown,
    Mute,
    Brightness(u8),
    Screenshot,
    Lock,
    Sleep,
}

pub struct WakeWord {
    pub phrase: String,
    pub sensitivity: f32,
    pub active: bool,
}

pub struct VoiceEngine {
    pub wake_words: Vec<WakeWord>,
    pub listening: bool,
    pub last_transcript: Option<String>,
    pub command_history: Vec<String>,
    pub confidence_threshold: f32,
}

impl VoiceEngine {
    pub fn new() -> Self {
        Self {
            wake_words: vec![WakeWord {
                phrase: String::from("hey aura"),
                sensitivity: 0.8,
                active: true,
            }],
            listening: false,
            last_transcript: None,
            command_history: Vec::new(),
            confidence_threshold: 0.7,
        }
    }

    pub fn start_listening(&mut self) {
        self.listening = true;
    }

    pub fn stop_listening(&mut self) {
        self.listening = false;
    }

    pub fn process_audio(&mut self, _samples: &AudioSample) -> Option<VoiceCommand> {
        None
    }

    pub fn parse_command(&mut self, text: &str) -> VoiceCommand {
        let lower = text.to_lowercase();
        self.command_history.push(String::from(text));
        self.last_transcript = Some(String::from(text));

        if lower.contains("volume up") {
            VoiceCommand::SystemControl(SystemAction::VolumeUp)
        } else if lower.contains("volume down") {
            VoiceCommand::SystemControl(SystemAction::VolumeDown)
        } else if lower.contains("mute") {
            VoiceCommand::SystemControl(SystemAction::Mute)
        } else if lower.contains("screenshot") {
            VoiceCommand::SystemControl(SystemAction::Screenshot)
        } else if lower.contains("lock") {
            VoiceCommand::SystemControl(SystemAction::Lock)
        } else if lower.starts_with("open ") || lower.starts_with("launch ") {
            let app = lower.replace("open ", "").replace("launch ", "");
            VoiceCommand::AppLaunch(app)
        } else {
            VoiceCommand::Text(String::from(text))
        }
    }

    pub fn add_wake_word(&mut self, phrase: &str, sensitivity: f32) {
        self.wake_words.push(WakeWord {
            phrase: String::from(phrase),
            sensitivity,
            active: true,
        });
    }
}
