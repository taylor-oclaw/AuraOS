extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Clean up the module
}

pub struct AuraLockScreen {
    password: String,
    attempts: u32,
    max_attempts: u32,
    locked: bool,
    messages: Vec<String>,
}

impl AuraLockScreen {
    pub fn new(password: &str, max_attempts: u32) -> Self {
        AuraLockScreen {
            password: String::from(password),
            attempts: 0,
            max_attempts,
            locked: false,
            messages: Vec::new(),
        }
    }

    pub fn authenticate(&mut self, input_password: &str) -> bool {
        if self.locked {
            return false;
        }

        if input_password == self.password {
            self.attempts = 0;
            true
        } else {
            self.attempts += 1;
            if self.attempts >= self.max_attempts {
                self.lock();
            }
            false
        }
    }

    pub fn lock(&mut self) {
        self.locked = true;
        self.messages.push(String::from("Screen locked due to too many failed attempts."));
    }

    pub fn unlock(&mut self, password: &str) -> bool {
        if password == self.password {
            self.locked = false;
            self.attempts = 0;
            self.messages.clear();
            true
        } else {
            false
        }
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }

    pub fn get_messages(&self) -> &Vec<String> {
        &self.messages
    }
}
