extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let mut message_react = MiniAppMessageReact::new();
    message_react.add_message(String::from("Hello, Kernel!"));
    message_react.add_message(String::from("This is a test message."));

    if let Some(msg) = message_react.get_last_message() {
        println!("Last message: {}", msg);
    }

    message_react.remove_message(0);

    if let Some(msg) = message_react.get_message_at(0) {
        println!("Message at index 0: {}", msg);
    } else {
        println!("No message at index 0.");
    }

    println!("Total messages: {}", message_react.message_count());
}

pub struct MiniAppMessageReact {
    messages: Vec<String>,
}

impl MiniAppMessageReact {
    pub fn new() -> Self {
        MiniAppMessageReact {
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
    }

    pub fn get_last_message(&self) -> Option<&String> {
        self.messages.last()
    }

    pub fn remove_message(&mut self, index: usize) -> Option<String> {
        if index < self.messages.len() {
            Some(self.messages.remove(index))
        } else {
            None
        }
    }

    pub fn get_message_at(&self, index: usize) -> Option<&String> {
        self.messages.get(index)
    }

    pub fn message_count(&self) -> usize {
        self.messages.len()
    }
}
