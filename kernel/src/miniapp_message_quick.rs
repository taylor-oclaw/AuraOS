extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let mut app = MiniAppMessageQuick::new();
    app.add_message("Hello, Kernel!");
    app.add_message("This is a test message.");
    app.display_messages();
    app.remove_message(0);
    app.display_messages();
}

pub struct MiniAppMessageQuick {
    messages: Vec<String>,
}

impl MiniAppMessageQuick {
    pub fn new() -> Self {
        MiniAppMessageQuick {
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: &str) {
        self.messages.push(String::from(message));
    }

    pub fn remove_message(&mut self, index: usize) -> Option<String> {
        if index < self.messages.len() {
            Some(self.messages.remove(index))
        } else {
            None
        }
    }

    pub fn get_message(&self, index: usize) -> Option<&String> {
        self.messages.get(index)
    }

    pub fn display_messages(&self) {
        for message in &self.messages {
            // Simulate printing to a kernel log or console
            unsafe { printk(message.as_ptr(), message.len()) };
        }
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }
}

// Dummy function to simulate printk
unsafe fn printk(ptr: *const u8, len: usize) {
    // This is a placeholder for actual kernel logging or console output
    // In a real kernel module, you would use the appropriate kernel API here
}
