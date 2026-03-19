extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum FocusMode {
    Work,
    Personal,
    Sleep,
    Meeting,
    Custom(String),
}

pub struct FocusModeConfig {
    pub mode: FocusMode,
    pub mute_notifications: bool,
    pub allowed_agents: Vec<u64>,
    pub blocked_apps: Vec<String>,
    pub auto_reply: Option<String>,
    pub schedule_start: Option<u64>,
    pub schedule_end: Option<u64>,
}

pub struct FocusManager {
    pub current_mode: FocusMode,
    pub configs: Vec<FocusModeConfig>,
    pub mode_history: Vec<(String, u64)>,
}

impl FocusManager {
    pub fn new() -> Self {
        Self {
            current_mode: FocusMode::Personal,
            configs: vec![
                FocusModeConfig {
                    mode: FocusMode::Work,
                    mute_notifications: true,
                    allowed_agents: Vec::new(),
                    blocked_apps: Vec::new(),
                    auto_reply: Some(String::from("I am currently in work mode")),
                    schedule_start: None,
                    schedule_end: None,
                },
                FocusModeConfig {
                    mode: FocusMode::Sleep,
                    mute_notifications: true,
                    allowed_agents: Vec::new(),
                    blocked_apps: Vec::new(),
                    auto_reply: Some(String::from("Sleeping - urgent only")),
                    schedule_start: None,
                    schedule_end: None,
                },
            ],
            mode_history: Vec::new(),
        }
    }

    pub fn set_mode(&mut self, mode: FocusMode) {
        self.mode_history.push((String::from("switch"), 0));
        self.current_mode = mode;
    }

    pub fn should_mute(&self) -> bool {
        match &self.current_mode {
            FocusMode::Work | FocusMode::Sleep | FocusMode::Meeting => true,
            _ => false,
        }
    }

    pub fn is_agent_allowed(&self, _agent_id: u64) -> bool {
        match &self.current_mode {
            FocusMode::Personal => true,
            _ => false,
        }
    }
}
