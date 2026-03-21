extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechGestureToText {
    // Example fields for demonstration purposes
    gestures: Vec<String>,
    speech_commands: Vec<String>,
}

impl SpeechGestureToText {
    pub fn new() -> Self {
        SpeechGestureToText {
            gestures: Vec::new(),
            speech_commands: Vec::new(),
        }
    }

    pub fn add_gesture(&mut self, gesture: String) {
        self.gestures.push(gesture);
    }

    pub fn add_speech_command(&mut self, command: String) {
        self.speech_commands.push(command);
    }

    pub fn get_gestures(&self) -> &Vec<String> {
        &self.gestures
    }

    pub fn get_speech_commands(&self) -> &Vec<String> {
        &self.speech_commands
    }

    pub fn recognize_gesture(&self, gesture: &str) -> Option<&String> {
        self.gestures.iter().find(|g| g == &gesture)
    }

    pub fn recognize_speech_command(&self, command: &str) -> Option<&String> {
        self.speech_commands.iter().find(|c| c == &command)
    }
}
