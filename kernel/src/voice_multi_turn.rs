extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let mut voice_system = VoiceMultiTurn::new();
    voice_system.add_user("Alice");
    voice_system.add_user("Bob");

    voice_system.record_message("Alice", "Hello Bob, how are you?");
    voice_system.record_message("Bob", "I'm doing well, Alice. How about you?");
    voice_system.record_message("Alice", "I'm good too. What have you been up to lately?");

    let messages = voice_system.get_messages_for_user("Bob");
    for message in messages {
        println!("Message: {}", message);
    }

    let users = voice_system.list_users();
    for user in users {
        println!("User: {}", user);
    }
}

pub struct VoiceMultiTurn {
    users: Vec<String>,
    messages: Vec<(String, String)>, // (sender, content)
}

impl VoiceMultiTurn {
    pub fn new() -> Self {
        VoiceMultiTurn {
            users: Vec::new(),
            messages: Vec::new(),
        }
    }

    pub fn add_user(&mut self, user: &str) {
        if !self.users.contains(&user.to_string()) {
            self.users.push(user.to_string());
        }
    }

    pub fn record_message(&mut self, sender: &str, content: &str) {
        if self.users.contains(&sender.to_string()) {
            self.messages.push((sender.to_string(), content.to_string()));
        }
    }

    pub fn get_messages_for_user(&self, user: &str) -> Vec<String> {
        self.messages
            .iter()
            .filter(|&&(ref s, _)| s == user)
            .map(|(_, ref c)| c.clone())
            .collect()
    }

    pub fn list_users(&self) -> Vec<String> {
        self.users.clone()
    }
}
