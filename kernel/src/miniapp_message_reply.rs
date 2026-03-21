extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let mut message_reply = MiniAppMessageReply::new();
    message_reply.add_message("Hello, Kernel!");
    message_reply.add_message("How are you?");
    println!("Messages: {:?}", message_reply.get_messages());
    println!("Last Message: {}", message_reply.get_last_message().unwrap_or(String::from("No messages")));
    println!("Total Messages: {}", message_reply.total_messages());
    message_reply.clear_messages();
    println!("Messages after clear: {:?}", message_reply.get_messages());
}

pub struct MiniAppMessageReply {
    messages: Vec<String>,
}

impl MiniAppMessageReply {
    pub fn new() -> Self {
        MiniAppMessageReply {
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: &str) {
        self.messages.push(message.to_string());
    }

    pub fn get_messages(&self) -> &Vec<String> {
        &self.messages
    }

    pub fn get_last_message(&self) -> Option<&String> {
        self.messages.last()
    }

    pub fn total_messages(&self) -> usize {
        self.messages.len()
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }
}
