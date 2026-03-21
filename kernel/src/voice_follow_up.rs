extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    let mut voice_follow_up = VoiceFollowUp::new();
    voice_follow_up.add_command("hello", String::from("Hello! How can I assist you today?"));
    voice_follow_up.add_command("goodbye", String::from("Goodbye! Have a great day."));
    voice_follow_up.add_command("help", String::from("I can help with various tasks. Just ask me something."));

    let response = voice_follow_up.get_response("hello");
    if let Some(response) = response {
    }

    voice_follow_up.remove_command("goodbye");

    let updated_response = voice_follow_up.get_response("goodbye");
    if updated_response.is_none() {
    }
}

pub struct VoiceFollowUp {
    commands: Vec<(String, String)>,
}

impl VoiceFollowUp {
    pub fn new() -> Self {
        VoiceFollowUp {
            commands: Vec::new(),
        }
    }

    pub fn add_command(&mut self, command: &str, response: String) {
        self.commands.push((String::from(command), response));
    }

    pub fn get_response(&self, command: &str) -> Option<&String> {
        for (cmd, resp) in &self.commands {
            if cmd == command {
                return Some(resp);
            }
        }
        None
    }

    pub fn remove_command(&mut self, command: &str) {
        self.commands.retain(|(cmd, _)| cmd != command);
    }

    pub fn list_commands(&self) -> Vec<&String> {
        self.commands.iter().map(|(cmd, _)| cmd).collect()
    }

    pub fn update_response(&mut self, command: &str, new_response: String) {
        if let Some(index) = self.commands.iter().position(|(cmd, _)| cmd == command) {
            self.commands[index].1 = new_response;
        }
    }
}
