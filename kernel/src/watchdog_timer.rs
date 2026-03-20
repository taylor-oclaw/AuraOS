extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct WatchdogTimer {
    name: String,
    timeout: u64,
    is_active: bool,
    reset_count: u32,
}

impl WatchdogTimer {
    pub fn new(name: &str, timeout: u64) -> Self {
        WatchdogTimer {
            name: String::from(name),
            timeout,
            is_active: false,
            reset_count: 0,
        }
    }

    pub fn start(&mut self) {
        if !self.is_active {
            self.is_active = true;
            // Simulate starting the timer
            println!("Watchdog Timer {} started with timeout {}", self.name, self.timeout);
        }
    }

    pub fn stop(&mut self) {
        if self.is_active {
            self.is_active = false;
            // Simulate stopping the timer
            println!("Watchdog Timer {} stopped", self.name);
        }
    }

    pub fn reset(&mut self) {
        if self.is_active {
            self.reset_count += 1;
            // Simulate resetting the timer
            println!("Watchdog Timer {} reset, count: {}", self.name, self.reset_count);
        }
    }

    pub fn get_timeout(&self) -> u64 {
        self.timeout
    }

    pub fn is_running(&self) -> bool {
        self.is_active
    }
}
