extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct AutoTaskSlackMessage {
    token: String,
    channel: String,
    message: String,
}

impl AutoTaskSlackMessage {
    pub fn new(token: &str, channel: &str, message: &str) -> Self {
        AutoTaskSlackMessage {
            token: String::from(token),
            channel: String::from(channel),
            message: String::from(message),
        }
    }

    pub fn set_token(&mut self, token: &str) {
        self.token = String::from(token);
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }

    pub fn set_channel(&mut self, channel: &str) {
        self.channel = String::from(channel);
    }

    pub fn get_channel(&self) -> &str {
        &self.channel
    }

    pub fn set_message(&mut self, message: &str) {
        self.message = String::from(message);
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }
}
