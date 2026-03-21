extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn tone_anger_careful_reply_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn tone_anger_careful_reply_exit() {
    // Cleanup logic for the module
}

pub struct ToneAngerCarefulReply {
    messages: Vec<String>,
}

impl ToneAngerCarefulReply {
    pub fn new() -> Self {
        ToneAngerCarefulReply {
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
    }

    pub fn get_messages(&self) -> &Vec<String> {
        &self.messages
    }

    pub fn remove_message(&mut self, index: usize) -> Option<String> {
        if index < self.messages.len() {
            Some(self.messages.remove(index))
        } else {
            None
        }
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }

    pub fn count_messages(&self) -> usize {
        self.messages.len()
    }
}

#[no_mangle]
pub extern "C" fn tone_anger_careful_reply_add_message(module: *mut ToneAngerCarefulReply, message: *const u8, length: usize) {
    unsafe {
        if let Some(module_ref) = module.as_mut() {
            let message_str = String::from_utf8_lossy(core::slice::from_raw_parts(message, length)).into_owned();
            module_ref.add_message(message_str);
        }
    }
}

#[no_mangle]
pub extern "C" fn tone_anger_careful_reply_remove_message(module: *mut ToneAngerCarefulReply, index: usize) -> Option<String> {
    unsafe {
        if let Some(module_ref) = module.as_mut() {
            module_ref.remove_message(index)
        } else {
            None
        }
    }
}

#[no_mangle]
pub extern "C" fn tone_anger_careful_reply_clear_messages(module: *mut ToneAngerCarefulReply) {
    unsafe {
        if let Some(module_ref) = module.as_mut() {
            module_ref.clear_messages();
        }
    }
}

#[no_mangle]
pub extern "C" fn tone_anger_careful_reply_count_messages(module: *const ToneAngerCarefulReply) -> usize {
    unsafe {
        if let Some(module_ref) = module.as_ref() {
            module_ref.count_messages()
        } else {
            0
        }
    }
}
