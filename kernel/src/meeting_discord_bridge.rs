extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn meeting_discord_bridge_init() {
    // Initialization logic for the module
}

pub extern "C" fn meeting_discord_bridge_exit() {
    // Cleanup logic for the module
}

pub struct MeetingDiscordBridge {
    users: Vec<String>,
    messages: Vec<String>,
}

impl MeetingDiscordBridge {
    pub fn new() -> Self {
        MeetingDiscordBridge {
            users: Vec::new(),
            messages: Vec::new(),
        }
    }

    pub fn add_user(&mut self, user: String) {
        self.users.push(user);
    }

    pub fn remove_user(&mut self, user: &str) -> bool {
        if let Some(index) = self.users.iter().position(|u| u == user) {
            self.users.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_users(&self) -> Vec<String> {
        self.users.clone()
    }

    pub fn send_message(&mut self, message: String) {
        self.messages.push(message);
    }

    pub fn get_messages(&self) -> Vec<String> {
        self.messages.clone()
    }
}
