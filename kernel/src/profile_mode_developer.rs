extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut profile = ProfileModeDeveloper::new();
    profile.enable_debug_mode();
    profile.set_user("developer");
    profile.log_event("Module initialized".to_string());
    profile.disable_debug_mode();
    loop {}
}

pub struct ProfileModeDeveloper {
    debug_mode: bool,
    user: String,
    events: Vec<String>,
}

impl ProfileModeDeveloper {
    pub fn new() -> Self {
        ProfileModeDeveloper {
            debug_mode: false,
            user: String::new(),
            events: Vec::new(),
        }
    }

    pub fn enable_debug_mode(&mut self) {
        self.debug_mode = true;
        self.log_event("Debug mode enabled".to_string());
    }

    pub fn disable_debug_mode(&mut self) {
        self.debug_mode = false;
        self.log_event("Debug mode disabled".to_string());
    }

    pub fn set_user(&mut self, user: String) {
        self.user = user;
        self.log_event(format!("User set to {}", self.user));
    }

    pub fn log_event(&mut self, event: String) {
        if self.debug_mode {
            self.events.push(event);
        }
    }

    pub fn get_events(&self) -> Vec<String> {
        self.events.clone()
    }
}
