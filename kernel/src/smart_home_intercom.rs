extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let intercom = SmartHomeIntercom::new();
    intercom.add_user("Alice".into());
    intercom.add_user("Bob".into());
    intercom.send_message("Hello, everyone!".into());
    intercom.list_users();
    intercom.remove_user("Alice".into());
    intercom.list_users();
}

pub struct SmartHomeIntercom {
    users: Vec<String>,
    messages: Vec<String>,
}

impl SmartHomeIntercom {
    pub fn new() -> Self {
        SmartHomeIntercom {
            users: Vec::new(),
            messages: Vec::new(),
        }
    }

    pub fn add_user(&mut self, name: String) {
        if !self.users.contains(&name) {
            self.users.push(name);
        }
    }

    pub fn remove_user(&mut self, name: String) {
        self.users.retain(|user| user != &name);
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
